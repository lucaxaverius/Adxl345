/* 
 * Copyright 2024 Luca Saverio Esposito, Università di Roma, Tor Vergata 
 * email: <lucasaverioesposito@gmail.com>
 *
 * This file is part of an "Rust Linux driver for the ADXL345 device".
 *
 * This driver is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 2 of the License, or (at your option)
 * any later version.
 *
 * This driver is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with Foobar.  If not, see <http://www.gnu.org/licenses/>.
 */

// structures.rs
pub(crate) mod constant;

use kernel::prelude::*;
use kernel::i2c::{I2CClient, I2CDriver};
use crate::fileops::utility::structures::constant::*; // Import the `constant` module for use in this file.
use kernel::chrdev::{Registration};
use kernel::error::code::{EINVAL};
use kernel::sync::{Arc, SpinLock};

/// Represents a single sample from the ADXL345 accelerometer,
/// containing X, Y, and Z axis data as 16-bit signed integers.
#[repr(C)]
#[derive(Copy, Clone)]
pub (crate) struct Adxl345Sample {
    pub (crate) x: i16,
    pub (crate) y: i16,
    pub (crate) z: i16,
}

impl Adxl345Sample {
    /// Creates a new `Adxl345Sample` with provided x, y, and z values.
    ///
    /// # Parameters
    /// - `x`: X-axis sample value
    /// - `y`: Y-axis sample value
    /// - `z`: Z-axis sample value
    ///
    /// # Returns
    /// A new instance of `Adxl345Sample`.
    pub (crate) const fn new(x: i16, y: i16, z: i16) -> Self {
        Adxl345Sample { x, y, z }
    }
}

/// Main structure for the ADXL345 accelerometer driver. It holds references to
/// the I2C client and device file state, as well as synchronization mechanisms
/// to handle concurrent access.
pub (crate) struct Adxl345 {
    pub (crate) client: I2CClient,                 // I2C client representing the ADXL345 device
    pub (crate) registration: Option<Pin<Box<Registration<1>>>>,  // Character device registration
}

unsafe impl Send for Adxl345 {}
unsafe impl Sync for Adxl345 {}



impl Adxl345 {
    /// Creates a new `Adxl345` instance with the provided I2C client.
    /// The char device driver isn't initialized here, it happens during device probe .
    ///
    /// # Parameters
    /// - `client`: I2C client associated with the ADXL345 device.
    ///
    /// # Returns
    /// A new instance of `Adxl345`.
    pub (crate) fn new(client: I2CClient) -> Self {
        Adxl345 {
            client,
            registration: None,
        }
    }

    /// Reads a byte from a specific register of the ADXL345 device.
    ///
    /// # Parameters
    /// - `reg_name`: The register name (or command) from which the byte should be read.
    ///
    /// # Returns
    /// - `Ok(u8)` containing the byte read from the register.
    /// - `Err(Error)` if an error occurs during the read operation.
    pub (crate) fn read_register(&self, reg_name: u8) -> Result<u8> {
        self.client.read_byte(reg_name)
    }

    /// Writes a byte to a specific register of the ADXL345 device.
    ///
    /// # Parameters
    /// - `reg_name`: The register name (or command) to which the byte should be written.
    /// - `value`: The byte value to be written to the register.
    ///
    /// # Returns
    /// - `Ok(())` if the write operation is successful.
    /// - `Err(Error)` if an error occurs during the write operation.
    pub (crate) fn write_register(&self, reg_name: u8, value: u8) -> Result<()> {
        self.client.write_byte(reg_name, value)
    }

    /// Checks if new data is ready from the ADXL345 device.
    ///
    /// # Returns
    /// - `Ok(1)` if data is ready.
    /// - `Ok(0)` if data is not ready.
    /// - `Err(Error)` if there is an I/O error during the read operation.
    pub (crate) fn data_ready(&self) -> Result<u8> {
        match self.read_register(ADXL345_REG_INT_SOURCE) {
            Ok(ret) if ret & 0x80 != 0 => Ok(1),
            Ok(_) => Ok(0),
            Err(e) => {
                pr_err!("failed to read INT_SOURCE register\n");
                Err(e)
            }
        }
    }

    /// Enables measurement mode on the ADXL345 device.
    ///
    /// # Returns
    /// - `Ok(())` if measurement mode is successfully enabled.
    /// - `Err(Error)` if an I/O error occurs during the process.
    ///
    /// Note: The device requires approximately 2ms to wake up after enabling.
    pub (crate) fn enable_measure(&self) -> Result<()> {
        // Read the current value of the POWER_CTL register
        let mut ret = match self.read_register(ADXL345_REG_POWER_CTL) {
            Ok(value) => value,
            Err(e) => {
                pr_err!("failed to enable measure\n");
                return Err(e);
            }
        };

        // Set the measurement bit (bit 3) to enable measurement mode
        ret |= 1 << 3;
        
        // Write the updated value back to the POWER_CTL register
        match self.write_register(ADXL345_REG_POWER_CTL, ret) {
            Ok(_) => Ok(()),
            Err(e) => {
                pr_err!("failed to enable measure\n");
                Err(e)
            }
        }
    }

    /// Disables measurement mode on the ADXL345 device.
    ///
    /// # Returns
    /// - `Ok(())` if measurement mode is successfully disabled.
    /// - `Err(Error)` if an I/O error occurs during the process.
    pub (crate) fn disable_measure(&self) -> Result<()> {
        // Read the current value of the POWER_CTL register
        let mut ret = match self.read_register(ADXL345_REG_POWER_CTL) {
            Ok(value) => value,
            Err(e) => {
                pr_err!("failed to disable measure\n");
                return Err(e);
            }
        };

        // Clear the measurement bit (bit 3) to disable measurement mode
        ret &= !(1 << 3);

        // Write the updated value back to the POWER_CTL register
        match self.write_register(ADXL345_REG_POWER_CTL, ret) {
            Ok(_) => Ok(()),
            Err(e) => {
                pr_err!("failed to disable measure\n");
                Err(e)
            }
        }
    }

    /// Sets the default configuration for the ADXL345 device.
    ///
    /// # Returns
    /// - `Ok(())` if the default configuration is successfully set.
    /// - `Err(Error)` if an I/O error occurs during the configuration process.
    pub (crate) fn set_default_config(&self) -> Result<()> {
        // Put device in standby mode
        self.write_register(ADXL345_REG_POWER_CTL, 0x00)
            .map_err(|e| {
                // map err is just a construct to map the error into another one.
                // It is used only to print some information message, and leave the original error.
                pr_err!("failed to set POWER_CTL to standby\n");
                e
            })?;

        // Disable device interrupts
        self.write_register(ADXL345_REG_INT_ENABLE, 0x00)
            .map_err(|e| {
                pr_err!("failed to disable interrupts\n");
                e
            })?;

        // Read and configure BW_RATE
        let mut value = self.read_register(ADXL345_REG_BW_RATE).map_err(|e| {
            pr_err!("failed to read BW_RATE register\n");
            e
        })?;

        // Log output data rate
        pr_debug!("Output data rate {} Hz\n", (value & 0xF) * 10);

        // Clear LOW_POWER bit
        value = value & 0xFF;
        value &= !(1 << 4);
        self.write_register(ADXL345_REG_BW_RATE, value).map_err(|e| {
            pr_err!("failed to configure BW_RATE register\n");
            e
        })?;

        // Set data format (full resolution, right justified, ±16g)
        self.write_register(ADXL345_REG_DATA_FORMAT, 0x0B).map_err(|e| {
            pr_err!("failed to set DATA_FORMAT\n");
            e
        })?;

        // Route all interrupts to INT1
        self.write_register(ADXL345_REG_INT_MAP, 0x00).map_err(|e| {
            pr_err!("failed to route interrupts to INT1\n");
            e
        })?;

        // Read and configure FIFO_CTL
        value = self.read_register(ADXL345_REG_FIFO_CTL).map_err(|e| {
            pr_err!("failed to read FIFO_CTL register\n");
            e
        })?;

        // Bypass FIFO
        value = value & 0xFF;
        value &= !(3 << 6);
        self.write_register(ADXL345_REG_FIFO_CTL, value).map_err(|e| {
            pr_err!("failed to configure FIFO_CTL register\n");
            e
        })?;

        Ok(())
    }

    /// Reads the x, y, and z axis data (6 bytes in total) from the ADXL345 device.
    ///
    /// # Returns
    /// - `Ok(Adxl345Sample)` if the data is successfully read and parsed.
    /// - `Err(Error)` if an I/O error occurs during the read operation.
    pub (crate) fn read_data(&self) -> Result<Adxl345Sample> {
        let mut data = [0u8; 6]; // Buffer to store the 6 bytes of data

        // Read 6 bytes starting from DATAX0 register
        match self.client.read_i2c_block(ADXL345_REG_DATAX0, 6, &mut data) {
            Ok(6) => {
                // Convert bytes to x, y, and z using little-endian to native format
                let x = i16::from_le_bytes([data[0], data[1]]) << 2;
                let y = i16::from_le_bytes([data[2], data[3]]) << 2;
                let z = i16::from_le_bytes([data[4], data[5]]) << 2;

                Ok(Adxl345Sample { x, y, z })
            }
            Ok(_) => {
                pr_err!("Incomplete data read\n");
                Err(EINVAL)
            }
            Err(e) => {
                pr_err!("Could not read block data\n");
                Err(e)
            }
        }
    }

    /// Getter function for the `client` field.
    pub (crate) fn client(&self) -> &I2CClient {
        &self.client
    }
}

// Define the main driver structure for ADXL345
pub (crate) struct Adxl345Driver {
    pub(crate) device: Arc<SpinLock<Adxl345>>,
    driver: Option<I2CDriver>,
    this_module: &'static ThisModule,
}

impl Adxl345Driver {
    /// Creates a new instance of `Adxl345Driver`.
    ///
    /// # Parameters
    /// - `device`: An `Arc` of a `SpinLock` containing an `Adxl345` instance,
    ///    representing the main device state for the ADXL345 accelerometer.
    /// - `module`: A reference to the current module (`ThisModule`) associated
    ///    with this driver. This is required for registering the char device associated
    ///    to the module.
    ///
    /// # Returns
    /// Returns a new `Adxl345Driver` instance with the provided device state,
    /// I2C driver, and module reference.
    pub (crate) fn new(device: Arc<SpinLock<Adxl345>>, module: &'static ThisModule) -> Self {
        // Create the new `Adxl345Driver` instance
        let adxl345driver = Self {
            device,
            driver: None,
            this_module: module,
        };

        // Return the driver instance
        adxl345driver
    }

    /// Getter for the `device` field
    pub (crate) fn device(&self) -> &Arc<SpinLock<Adxl345>> {
        &self.device
    }

    /// Getter for the `driver` field
    pub (crate) fn driver(&self) -> Option<&I2CDriver> {
        self.driver.as_ref()
    }
    
    /// Returns a reference to the `ThisModule` instance associated with this driver.
    ///
    /// # Returns
    /// A reference to `ThisModule`, providing access to module-specific information.
    pub (crate) fn this_module(&self) -> &'static ThisModule {
        self.this_module
    }

    /// Sets the `driver` field to a new `I2CDriver` instance.
    ///
    /// # Parameters
    /// - `new_driver`: The new `I2CDriver` instance to assign to the `driver` field.
    fn set_driver(&mut self, new_driver: I2CDriver) {
        self.driver = Some(new_driver);
    }

    pub (crate) fn set_driver_pinned(self: Pin<&mut Self>, new_driver: I2CDriver) {
        // SAFETY: This is safe because we are modifying a field of a pinned structure,
        // which does not affect the pinning guarantees.
        unsafe{self.get_unchecked_mut()}.set_driver(new_driver);
    }
}




