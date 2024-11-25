// driver.rs

//! Module for I2C driver representation.
//!
//! This module provides structures and traits for creating and managing I2C drivers.

use crate::prelude::*;
use crate::bindings;
use core::ffi::{c_void, c_int};
use crate::i2c::client::I2CClient;
use crate::error::{to_result};
use core::result::Result as CoreResult;

/// Represents an I2C driver.
///
/// An `I2CDriver` contains the necessary information to register and manage an I2C driver in the kernel.
///
/// # Invariants
///
/// - `driver` is valid, non-null, and points to a properly allocated and initialized
///   `bindings::i2c_driver` structure.
/// - The lifetime of the `driver` pointer is managed externally (e.g. explicit deallocation in `remove_driver`).
/// - Concurrent access to the `driver` is safe because the underlying I2C driver structure is
///   designed to handle it (e.g., through synchronization in the kernel).
pub struct I2CDriver {
    /// Pointer to the underlying `i2c_driver` struct.
    driver: *mut bindings::i2c_driver,
}

impl I2CDriver {
    /// Registers the I2C driver with the kernel.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if registration is successful.
    /// * `Err(Error)` if registration fails.
    pub fn add_driver(&self) -> Result<()> {
        if self.driver.is_null() {
            return Err(EINVAL);
        } 
        let ret = unsafe { bindings::i2c_add_driver(self.driver) };
        to_result(ret)
    }

    /// Deregisters the I2C driver from the kernel and free the heap.
    ///
    /// It must be called in the Drop trait of the kernel module.
    pub fn remove_driver(&self) {
        if self.driver.is_null() {
            pr_info!("WARNING!!! Called remove driver to null ptr !!!");
            return;
        } 
        unsafe { 
            bindings::i2c_del_driver(self.driver);
            // Convert the raw pointer back to a Box so that Rust can properly deallocate it

            drop(Box::from_raw(self.driver));
        };
    
    }
}

// SAFETY: `I2CDriver` holds a pointer to a C `i2c_driver` struct, managed by the kernel, which is
// safe to be sent across threads because it is initialized before any thread access and is only
// modified in ways that the kernel's I2C subsystem supports for concurrency.
unsafe impl Send for I2CDriver {}

// SAFETY: `I2CDriver` is safe to share across threads because the kernel manages concurrency
// for the `i2c_driver` pointer, ensuring that the structure it points to can handle concurrent access.
unsafe impl Sync for I2CDriver {}

/// Builder for creating an `I2CDriver` instance.
///
/// Provides a convenient way to construct an `I2CDriver` with optional parameters.
pub struct I2CDriverBuilder<T: I2CDriverCallbacks> {
    // Fields for driver configuration
    class: Option<u32>,
    driver: bindings::device_driver,
    id_table: *const bindings::i2c_device_id,
    address_list: Option<*const u16>,
    clients: Option<bindings::list_head>,
    flags: Option<u32>,

    _marker: core::marker::PhantomData<T>, // Marker for the callback trait type
}

impl<T: I2CDriverCallbacks> I2CDriverBuilder<T> {
    /// Creates a new `I2CDriverBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `driver_name` - Name of the driver.
    /// * `owner` - Pointer to the module owning this driver.
    /// * `probe` - Probe callback function.
    /// * `remove` - Remove callback function.
    /// * `id_table` - Pointer to the device ID table.
   /// Creates a new `I2CDriverBuilder` for the specified `I2CDriverCallbacks` implementation.
    pub fn new(id_table: *const bindings::i2c_device_id, 
        driver_name: *const i8,
        owner: *mut bindings::module) -> Self {
        Self {
            class: None,
            driver: bindings::device_driver {
                name: driver_name,
                owner,
                ..Default::default()
            },
            id_table,
            address_list: None,
            clients: None,
            flags: None,
            _marker: core::marker::PhantomData,
        }
    }

    /// Sets the device class for the driver.
    pub fn class(mut self, class: u32) -> Self {
        self.class = Some(class);
        self
    }

    /// Sets the address list for device detection.
    pub fn address_list(mut self, address_list: *const u16) -> Self {
        self.address_list = Some(address_list);
        self
    }

    /// Sets the flags for the driver.
    pub fn clients(mut self, clients: bindings::list_head) -> Self {
        self.clients = Some(clients);
        self
    }

    /// Sets the flags for the driver.
    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }
    
    /// Builds and returns an `I2CDriver` instance.
    ///
    /// # Returns
    ///
    /// * `Ok(I2CDriver)` if the driver is successfully built.
    /// * `Err(Error)` if driver creation fails.
    pub fn build(self) -> Result<I2CDriver> {
        // Use `I2CDriverVtable` to obtain the C-compatible callbacks
        let driver = bindings::i2c_driver {
            driver: self.driver,
            // Initialize the `probe` union field with the `probe_callback` from `I2CDriverVtable`.
            __bindgen_anon_1: bindings::i2c_driver__bindgen_ty_1 {
                probe: Some(I2CDriverVtable::<T>::probe_callback),
            },
            remove: Some(I2CDriverVtable::<T>::remove_callback),
            shutdown: Some(I2CDriverVtable::<T>::shutdown_callback),
            alert: Some(I2CDriverVtable::<T>::alert_callback),
            command: Some(I2CDriverVtable::<T>::command_callback),
            detect: Some(I2CDriverVtable::<T>::detect_callback),
            id_table: self.id_table,
            class: self.class.unwrap_or(0),
            address_list: self.address_list.unwrap_or(core::ptr::null()),
            clients: self.clients.unwrap_or(bindings::list_head {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            }),
            flags: self.flags.unwrap_or(0),
        };

        // Box the driver to allocate it on the heap and get a stable pointer
        let driver_ptr = Box::into_raw(Box::try_new(driver)?);

        Ok(I2CDriver { driver: driver_ptr })
    }
}

/// Trait representing the essential callbacks for an I2C driver.
///
/// Implement this trait to define the behavior of your I2C driver.
///
/// # Safety:
///
/// The `I2CDriverCallbacks` trait is required to implement both `Send` and `Sync`, to be implemented in a static context.
/// Implementors of this trait are responsible for ensuring that their internal state adheres to
/// Rust's concurrency guarantees, making the `Send + Sync` markers appropriate.
///
pub trait I2CDriverCallbacks: Send + Sync {
    /// Called when the driver is bound to an I2C device.
    ///
    /// # Arguments
    ///
    /// * `client` - The `I2CClient` representing the device.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if probing is successful.
    /// * `Err(c_int)` if probing fails.
    fn probe(&self, client: &I2CClient) -> Result<()>;

    /// Called when the driver is unbound from an I2C device.
    ///
    /// # Arguments
    ///
    /// * `client` - The `I2CClient` representing the device.
    fn remove(&self, client: &I2CClient);

    /// Optional: Called during device shutdown.
    ///
    /// Default implementation does nothing.
    fn shutdown(&self, _client: &I2CClient) {
        pr_info!("I2C Shutdown called\n");
    }

    /// Optional: Called on I2C alerts.
    ///
    /// Default implementation does nothing.
    fn alert(
        &self,
        _client: &I2CClient,
        _protocol: bindings::i2c_alert_protocol,
        _data: u32,
    ) {
        pr_info!("I2C Alert called\n");
    }

    /// Optional: Handles custom I2C commands.
    ///
    /// Default implementation does nothing.
    fn command(
        &self,
        _client: &I2CClient,
        _cmd: u32,
        _arg: *mut c_void,
    ) -> Result<()> {
        pr_info!("I2C Command called\n");
        Ok(())
    }

    /// Optional: Performs device detection.
    ///
    /// Default implementation does nothing.
    fn detect(
        &self,
        _client: &I2CClient,
        _info: *mut bindings::i2c_board_info,
    ) -> Result<()> {
        pr_info!("I2C Detect called\n");
        Ok(())
    }
}


/// A virtual table (vtable) for I2C driver callbacks, providing an interface
/// to bridge C-compatible function pointers with Rust implementations.
///
/// The `I2CDriverVtable` structure is designed to work with `I2CDriverCallbacks`
/// implementations by offering C-compatible callbacks (such as `probe` and `remove`)
/// that can be used in the Linux kernel's I2C driver registration process. It allows
/// Rust implementations of these callbacks to be seamlessly invoked from C code.
///
/// # Type Parameters
///
/// * `T` - A type that implements the `I2CDriverCallbacks` trait, defining the Rust
///   callbacks for handling I2C device events such as probing and removal.
///
/// # Safety
///
/// This structure contains C function pointers that are intended to interface with
/// the Linux kernel. These function pointers must be set correctly and handle
/// safety requirements for interactions between C and Rust code.
pub struct I2CDriverVtable<T: I2CDriverCallbacks> {
    _marker: core::marker::PhantomData<T>,
}


impl<T: I2CDriverCallbacks> I2CDriverVtable<T> {
    /// Helper function to retrieve the driver instance stored in `clientdata`.
    ///
    /// # Safety
    /// This function assumes that the `clientdata` has been set correctly
    /// with a valid pointer to `T`.
    fn get_driver_instance(client: &I2CClient) -> CoreResult<&T, Error> {
        // Retrieve the stored driver instance from the client data
        let driver_instance_ptr = unsafe { client.get_clientdata::<T>() };

        // Check if the driver instance pointer is null
        if driver_instance_ptr.is_null() {
            pr_err!("Driver instance not set in client data; probe may have failed.");
            return Err(EINVAL); // Return an error code if no instance is set
        }

        // Convert the raw pointer back to a reference to `T`
        let driver_instance = unsafe { &*driver_instance_ptr };

        Ok(driver_instance)
    }

    /// Extern "C" probe callback that calls the Rust `probe` method on the
    /// provided `I2CDriverCallbacks` implementation.
    unsafe extern "C" fn probe_callback(client: *mut bindings::i2c_client) -> c_int {
        //from_kernel_result!{        
            // Convert the raw pointer to an I2CClient instance
            let client = unsafe{I2CClient::from_raw_ptr(client)};
            //pr_info!("Called the C probe callback");

            // Retrieve the driver instance from clientdata
            match Self::get_driver_instance(&client) {
                Ok(driver_instance) => {
                   // Call the `probe` method on the retrieved instance of T
                   //pr_info!("Adxl345Driver correctly found {:p}",driver_instance);
                    match driver_instance.probe(&client) {
                        Ok(_) => 0,
                        Err(e) => e.to_kernel_errno(),
                    }
                }
                Err(err) => {
                    pr_err!("Failed to retrieve driver instance in probe callback: {:?}", err);
                    err.to_kernel_errno()
                }
            }
        //}
    }

    /// Extern "C" remove callback that calls the Rust `remove` method on the
    /// provided `I2CDriverCallbacks` implementation.
    unsafe extern "C" fn remove_callback(client: *mut bindings::i2c_client) {
        // Convert the raw pointer to an I2CClient instance
        let client = unsafe{I2CClient::from_raw_ptr(client)};

        // Retrieve the driver instance from clientdata
        match Self::get_driver_instance(&client) {
            Ok(driver_instance) => {
                // Call the `remove` method on the driver instance
                driver_instance.remove(&client);

                // Clear the `i2cclient data` to avoid any dangling pointers.
                client.free_clientdata();
            }
            Err(err) => {
                pr_err!("Failed to retrieve driver instance in remove callback: {:?}", err);
            }
        }
    }

    /// Extern "C" shutdown callback that is triggered when the system is shutting down.
    /// 
    /// This optional function can be provided to handle device-specific shutdown logic.
    unsafe extern "C" fn shutdown_callback(client: *mut bindings::i2c_client) {
        let client = unsafe{I2CClient::from_raw_ptr(client)};
        match Self::get_driver_instance(&client){
            Ok(driver_instance) => {
                driver_instance.shutdown(&client);
            }
            Err(err) => {
                pr_err!("Failed to retrieve driver instance in shutdown callback: {:?}", err);
            }
        }

    }

    /// Extern "C" alert callback that is triggered on I2C alerts.
    /// 
    /// This optional function is called when an I2C alert occurs, typically used in SMBus.
    unsafe extern "C" fn alert_callback(
        client: *mut bindings::i2c_client,
        protocol: bindings::i2c_alert_protocol,
        data: u32,
    ) {
        let client = unsafe{I2CClient::from_raw_ptr(client)};
        match Self::get_driver_instance(&client){
            Ok(driver_instance) => {
                driver_instance.alert(&client, protocol, data);
            }
            Err(err) => {
                pr_err!("Failed to retrieve driver instance in alert callback: {:?}", err);
            }
        }
    }

    /// Extern "C" command callback that is triggered for custom I2C commands.
    /// 
    /// This optional function allows custom commands to be sent to the I2C device.
    unsafe extern "C" fn command_callback(
        client: *mut bindings::i2c_client,
        cmd: u32,
        arg: *mut core::ffi::c_void,
    ) -> i32 {
        let client = unsafe{I2CClient::from_raw_ptr(client)};
        // Retrieve the driver instance from clientdata
        match Self::get_driver_instance(&client) {
            Ok(driver_instance) => {
                // Call the `probe` method on the retrieved instance of T
                match driver_instance.command(&client, cmd, arg) {
                    Ok(_) => 0,
                    Err(e) => e.to_kernel_errno(),
                }
            }
            Err(err) => {
                pr_err!("Failed to retrieve driver instance in command callback: {:?}", err);
                err.to_kernel_errno()
            }
        }
    }

    /// Extern "C" detect callback that is triggered for device detection on the I2C bus.
    /// 
    /// This optional function can be used to detect devices on the I2C bus that do not
    /// explicitly announce their presence.
    unsafe extern "C" fn detect_callback(
        client: *mut bindings::i2c_client,
        info: *mut bindings::i2c_board_info,
    ) -> i32 {
        let client = unsafe{I2CClient::from_raw_ptr(client)};
        // Retrieve the driver instance from clientdata
        match Self::get_driver_instance(&client) {
            Ok(driver_instance) => {
                // Call the `probe` method on the retrieved instance of T
                match driver_instance.detect(&client, info) {
                    Ok(_) => 0,
                    Err(e) => e.to_kernel_errno(),
                }
            }
            Err(err) => {
                pr_err!("Failed to retrieve driver instance in detect callback: {:?}", err);
                err.to_kernel_errno()
            }
        }
    }
}