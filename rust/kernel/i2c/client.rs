// client.rs

//! Module for I2C client representation.
//!
//! This module provides the `I2CClient` struct, representing an I2C slave device,
//! and methods for communicating with the device.

use crate::prelude::*;
use crate::bindings;
use core::ffi::{c_char};
use crate::i2c::adapter::I2CAdapter;
use crate::i2c::board_info::I2CBoardInfo;
use crate::error::{to_result,from_kernel_err_ptr};

/// Represents an I2C client device.
///
/// An `I2CClient` is used to communicate with a specific I2C slave device on the bus.
///
/// # Invariants
/// - `ptr` is valid, non-null, and points to a properly allocated and initialized
///   `bindings::i2c_client` structure.
/// - `owned` indicates whether the `I2CClient` is responsible for unregistering the client in
///   its `Drop` implementation.
pub struct I2CClient {
    /// Pointer to the underlying `i2c_client` struct.
    ptr: *mut bindings::i2c_client,
    /// Ownership indicator, set at initialization and never changed.
    owned: bool,
}

// SAFETY:
// - `I2CClient` holds a pointer to a C `i2c_client`, managed entirely by the kernel.
// - The `ptr` field is private, ensuring that it cannot be directly accessed or modified 
//   outside of the `I2CClient` implementation. All interactions are routed through controlled APIs.
// - The kernel's I2C subsystem uses internal synchronization mechanisms (e.g., a mutex) to 
//   ensure safe concurrent access to the underlying `i2c_client` and its operations.
unsafe impl Send for I2CClient {}

// SAFETY:
// - `I2CClient` holds a pointer to a C `i2c_client`, which is safe to share across threads 
//   due to the kernel's use of internal synchronization (e.g., mutex) for I2C operations.
// - Encapsulation of `ptr` further ensures that references cannot be misused outside of 
//   the `I2CClient` implementation.
unsafe impl Sync for I2CClient {}


impl I2CClient {
    /// Attempts to create a new `I2CClient` device for the specified adapter and board info.
    ///
    /// # Parameters
    /// - `adapter`: Reference to the I2C adapter to which the device will be attached.
    /// - `board_info`: Information about the board to assist in creating the client.
    ///
    /// # Returns
    /// A result containing either a new `I2CClient` instance or an error.
    ///
    /// # Constraint
    /// When the device is created with this function, the device deallocation will be managed automatically
    /// by the drop trait.
    pub fn new_client_device(adapter: &I2CAdapter, board_info: &I2CBoardInfo) -> Result<Self> {
        // Attempt to create a new client device and handle the error pointer if returned.
        let client_ptr = 
            match unsafe {from_kernel_err_ptr(bindings::i2c_new_client_device(adapter.as_ptr(), board_info.as_ptr()))} {
                Ok(ptr) => ptr,
                Err(e) => {
                    pr_err!("Failed to create i2c_client: error code {:?}", e);
                    return Err(e);
                },
            };
      
        // Return the wrapped `I2CClient` instance if successful
        Ok(Self {
            ptr: client_ptr,
            owned: true,
        })
    }
    

    /// Creates an `I2CClient` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is valid.
    pub unsafe fn from_raw_ptr(ptr: *mut bindings::i2c_client) -> Self {
        Self { ptr, owned: false }
    }


    /// Sets the client data for this `I2CClient`.
    /// # Example
    /// 
    /// let mut driver_instance = MyDriver::new();
    /// i2c_client.set_clientdata(&mut driver_instance);
    /// 
    pub fn set_clientdata<T>(&self, data: &mut T) {
        unsafe { bindings::i2c_set_clientdata(self.ptr, data as *mut T as *mut core::ffi::c_void) };
    }

    /// Free client data for this `I2CClient` by putting a void pointer inside of it.
    /// # Example
    /// 
    /// let mut driver_instance = MyDriver::new();
    /// i2c_client.set_clientdata(&mut driver_instance);
    /// 
    pub fn free_clientdata(&self) {
        unsafe { bindings::i2c_set_clientdata(self.ptr, core::ptr::null_mut()) };
    }

    /// Gets the client data for this `I2CClient` and casts it to the specified type `T`.
    ///
    /// # Safety
    /// This function is `unsafe` because the caller must ensure that the type `T` matches
    /// the type of the data stored in the client data. Mismatched types can result in undefined behavior.
    ///
    /// # Example
    /// ```rust
    /// let driver_instance: &mut MyDriver = unsafe { i2c_client.get_clientdata::<MyDriver>() };
    /// ```
    pub unsafe fn get_clientdata<T>(&self) -> *mut T {
        (unsafe{bindings::i2c_get_clientdata(self.ptr)}) as *mut T
    }

    /// Sends data to the I2C client device.
    ///
    /// # Arguments
    ///
    /// * `buf` - A byte slice containing the data to send.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` indicating the number of bytes sent.
    /// * `Err(Error)` if the send operation fails.
    pub fn master_send(&self, buf: &[c_char]) -> Result<usize> {
        if buf.len() > u16::MAX as usize {
            return Err(EINVAL);
        }
        let ret = unsafe { bindings::i2c_master_send(self.ptr, buf.as_ptr(), buf.len() as i32) };
        to_result(ret).map(|_| ret as usize)
    }

    /// Receives data from the I2C client device.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable byte slice to store the received data.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` indicating the number of bytes received.
    /// * `Err(Error)` if the receive operation fails.
    pub fn master_recv(&self, buf: &mut [c_char]) -> Result<usize> {
        if buf.len() > u16::MAX as usize {
            return Err(EINVAL);
        }
        let ret =
            unsafe { bindings::i2c_master_recv(self.ptr, buf.as_mut_ptr(), buf.len() as i32) };
        to_result(ret).map(|_| ret as usize)
    }

    /// This executes the SMBus "send byte" protocol. 
    /// Writes a single byte to the I2C client device without specifying a device register.
    /// Some devices are so simple that this interface is enough; 
    /// for others, it is a shorthand if you want to read the same register as in the previous SMBus command.
    /// 
    /// # Arguments
    /// * `value` - The byte value to be written.
    /// 
    /// # Returns
    /// * `Ok(())` if the byte is successfully written.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn send_byte(&self, value: u8) -> Result<()> {
        let ret = unsafe { bindings::i2c_smbus_write_byte(self.ptr, value) };
        to_result(ret)
    }

    /// This executes the SMBus "receive byte" protocol.
    /// Reads a single byte from the I2C client device without specifying a device register. 
    /// Some devices are so simple that this interface is enough; 
    /// for others, it is a shorthand if you want to read the same register as in the previous SMBus command.
    ///
    /// # Returns
    /// * `Ok(u8)` if the byte is successfully read.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn receive_byte(&self) -> Result<u8> {
        let ret = unsafe { bindings::i2c_smbus_read_byte(self.ptr) };
        if ret < 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(ret as u8)
        }
    }

    /// This executes the SMBus "write byte" protocol with a command.
    /// Writes a byte to a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command to which the byte should be written.
    /// * `value` - The byte value to be written.
    ///
    /// # Returns
    /// * `Ok(())` if the byte is successfully written.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn write_byte(&self, command: u8, value: u8) -> Result<()> {
        let ret = unsafe { bindings::i2c_smbus_write_byte_data(self.ptr, command, value) };
        to_result(ret)
    }

    /// This executes the SMBus "read byte" protocol with a command.
    /// Reads a byte from a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command from which the byte should be read.
    ///
    /// # Returns
    /// * `Ok(u8)` if the byte is successfully read.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn read_byte(&self, command: u8) -> Result<u8> {
        let ret = unsafe { bindings::i2c_smbus_read_byte_data(self.ptr, command) };
        if ret < 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(ret as u8)
        }
    }

    /// This executes the SMBus "write word" protocol with a command.
    /// Writes a word to a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command to which the word should be written.
    /// * `value` - The word value to be written.
    ///
    /// # Returns
    /// * `Ok(())` if the word is successfully written.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn write_word(&self, command: u8, value: u16) -> Result<()> {
        let ret = unsafe { bindings::i2c_smbus_write_word_data(self.ptr, command, value) };
        to_result(ret)
    }

    /// This executes the SMBus "read word" protocol with a command.
    /// Reads a word from a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command from which the word should be read.
    ///
    /// # Returns
    /// * `Ok(u16)` if the word is successfully read.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn read_word(&self, command: u8) -> Result<u16> {
        let ret = unsafe { bindings::i2c_smbus_read_word_data(self.ptr, command) };
        if ret < 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(ret as u16)
        }
    }

    /// This executes the SMBus "block write" protocol with a command.
    /// Writes a block of data to a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command to which the block should be written.
    /// * `values` - The block of data to be written (maximum 32 bytes).
    ///
    /// # Returns
    /// * `Ok(())` if the block is successfully written.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn write_block(&self, command: u8, values: &[u8]) -> Result<()> {
        if values.len() > 32 {
            pr_err!("Can't write more that 32 bytes ");
            return Err(EINVAL);
        }
        let ret = unsafe {
            bindings::i2c_smbus_write_block_data(
                self.ptr,
                command,
                values.len() as u8,
                values.as_ptr() as *const u8,
            )
        };
        to_result(ret)
    }

    /// This executes the SMBus "block read" protocol with a command.
    /// Reads a block of data from a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command from which the block should be read.
    /// * `buf` - A mutable buffer (`&mut [u8]`) to store the data read from the slave. Maximum block size is 32 bytes.
    ///
    /// # Returns
    /// * `Ok(usize)` if the block is successfully read, indicating the number of bytes read.
    /// * `Err(Error)` if an error occurs during transmission.
    /// 
    /// # Warning
    /// Note that using this function requires that the client's adapter support
    /// the I2C_FUNC_SMBUS_READ_BLOCK_DATA functionality.  Not all adapter drivers
    /// support this; its emulation through I2C messaging relies on a specific
    /// mechanism (I2C_M_RECV_LEN) which may not be implemented.
    /// In this case use read_i2c_block.
    pub fn read_block(&self, command: u8, buf: &mut [u8]) -> Result<usize> {
        // Ensure the buffer length does not exceed the maximum block size (32 bytes).
        if buf.len() > 32 {
            pr_err!("Can't read more that 32 bytes ");
            return Err(EINVAL);
        }

        let ret = unsafe {
            bindings::i2c_smbus_read_block_data(
                self.ptr,
                command,
                buf.as_mut_ptr(),
            )
        };

        if ret < 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(ret as usize)
        }
    }

    /// This executes the SMBus "block read" protocol with a command.
    /// Reads a block of data from a specific register (command) of the I2C client device.
    ///
    /// # Arguments
    /// * `command` - The register/command from which the block should be read.
    /// * `len` - The number of bytes that should be read.
    /// * `buf` - A mutable buffer (`&mut [u8]`) to store the data read from the slave. Maximum block size is 32 bytes.
    ///
    /// # Returns
    /// * `Ok(usize)` if the block is successfully read, indicating the number of bytes read.
    /// * `Err(Error)` if an error occurs during transmission.
    pub fn read_i2c_block(&self, command: u8, len: u8, buf: &mut [u8]) -> Result<usize> {
        // Ensure the length does not exceed the maximum block size (32 bytes).
        if  len > 32 {
            pr_err!("Can't read more that 32 bytes ");
            return Err(EINVAL);
        }
        
        let ret = unsafe {
            bindings::i2c_smbus_read_i2c_block_data(
                self.ptr,
                command,
                len,
                buf.as_mut_ptr(),
            )
        };

        if ret < 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(ret as usize)
        }
    }
    
}

impl Drop for I2CClient {
    fn drop(&mut self) {
        if self.owned {
            // Unregister the I2C client device.
            unsafe { bindings::i2c_unregister_device(self.ptr) };
        }
    }
}
