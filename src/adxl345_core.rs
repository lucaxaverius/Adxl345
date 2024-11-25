// SPDX-License-Identifier: GPL-2.0-only

//! Driver for the ADXL345 accelerometer.

/* 
 * Copyright 2024 Luca Saverio Esposito, Università di Roma, Tor Vergata 
 * email: <lucasaverioesposito@gmail.com>
 *
 * This file is code of the driver for "Rust Linux driver for the ADXL345 device".
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

// adxl345_core.rs

// Register the module
module! {
    type: Adxl345Module,
    name: "adxl345",
    author: "Luca Saverio Esposito",
    description: "ADXL345 I2C driver in Rust",
    license: "GPL",
}

mod fileops;

use kernel::prelude::*;
use kernel::sync::{Arc,SpinLock};
use kernel::i2c::*;
use kernel::{i2c_module_device_table,spinlock_init};
use crate::fileops::utility::structures::constant::*;
use crate::fileops::utility::structures::{Adxl345Driver, Adxl345};
use crate::fileops::utility::{adxl345_device_init,adxl345_device_clean};
use crate::fileops::{adxl345_chardev_add, DEVICE_PTR};

// Define the I2C board information with device name and address.
static ADXL345_BOARD_INFO: I2CBoardInfo = I2CBoardInfo::new(DR_NAME, ADXL345_I2C_ADDR); // 0x1D is the address for ADXL345


// Define the I2C device ID table for this driver.
// This exposes the device IDs to the kernel so the driver can be matched with compatible devices.
const ID_TABLE_LEN: usize = 2;
static ADXL345_ID_TABLE: [I2CDeviceID; ID_TABLE_LEN] = [
    I2CDeviceID::new(DR_NAME, 0),
    I2CDeviceID::new(b"", 0), // Empty entry to mark the end of the table
];

// Expose the device table to the kernel module loader.
i2c_module_device_table!(ADXL345_ID_TABLE, ID_TABLE_LEN);


impl I2CDriverCallbacks for Adxl345Driver{
    fn probe(&self, _client: &I2CClient) -> Result {
        pr_info!("ADXL345 probe function called for device\n");
        
        {
            // Clone the Ref to the device (so increment the ref counter by one)
            let device = self.device().clone();   
            // Initialize the device (implement this method in `Adxl345`)
            adxl345_device_init(device).map_err(|_| EIO).expect("Failed Device initialization");
        }
        

        // Register the character device
        let registration = adxl345_chardev_add(
            CStr::from_bytes_with_nul(DR_NAME_WN).unwrap(),
            0, // Starting minor number
            self.this_module(),
        ).expect("Failed during chard dev registration");

        //pr_info!("Registration address: {:p}", registration);
        pr_info!("adxl345driver address {:p} \n", self);

        // Assign the `registration` field in `Adxl345`
        {
            // Clone the Ref to the device (so increment the ref counter by one)
            let device = self.device().clone(); 
            let mut device_lock = device.lock();
            device_lock.registration = Some(registration);
        }

        let device_arc = self.device.clone();
        // Save into the global variable for fileops
        unsafe{DEVICE_PTR =  Some(device_arc)};
        Ok(())
    }

    fn remove(&self, _client: &I2CClient){
        pr_info!("ADXL345 remove function called for device\n");

        // Clone the Ref to the device (so take a increment the ref counter by one)
        {
            let device = self.device().clone(); 

            // Attempt to clean up the device and log any errors
            if let Err(e) = adxl345_device_clean(device) {
                pr_err!("Failed to clean up ADXL345 device: {:?}", e);
            }
        }

        // Drop the Registration to deregister the character device
        {   
            let device = self.device().clone(); 
            let mut adxl = device.lock();
            
            // Deregisters the device automatically when `None` is assigned cause it's deallocated 
            // ando so the Drop trait is called.
            adxl.registration = None; 

        }
        
        // The data inside i2c-client are automatically dropped by the remove_callback
        
        // Clean up the global pointer:
        unsafe {
            DEVICE_PTR = None;
        }
        pr_info!("ADXL345 device successfully removed\n");
    }
}

struct Adxl345Module{
    the_driver: Pin<Box<Adxl345Driver>>,
}

impl kernel::Module for Adxl345Module {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("ADXL345 Rust driver initializing\n");

        // Initialize I2C adapter and create a new device
        let i2c_adapter = I2CAdapter::get_from_bus_number(ADXL345_I2C_ADAPTER).expect("Can't get the adapter"); 
        
        // This i2c_client instance is owned by Rust subsystem, so will be dropped
        // automatically when the module will be removed by the drop trait of I2CClient struct.
        let i2c_client = I2CClient::new_client_device(&i2c_adapter, &ADXL345_BOARD_INFO).expect("Cant take I2C-Client");
        
        let mut spin_adxl345 = unsafe{SpinLock::new(Adxl345::new(i2c_client))};

        // Init the spinlock
        spinlock_init!(unsafe { Pin::new_unchecked(&mut spin_adxl345)}, "adxl345");

        // Create the shared `Adxl345` instance wrapped in an `Arc`
        let device = Arc::try_new(spin_adxl345).expect("Failed during Arc creation");

        // Pin ensure that the driver doesn't move, this constraint is mandatory due the 
        // necessity of retrieving driver with i2c_get_clientdata.
        let mut adxl345driver = Pin::from(Box::try_new(Adxl345Driver::new(device, module))
        .expect("Failed to allocate Adxl345Driver"));

        {    
            // Is mandatory to take all the steps separately, otherwise the borrow checker cries :/
            //let adxl_device = adxl345driver.device.clone();
            let adxl_device = adxl345driver.device.clone();
            let adxl_lock = adxl_device.lock();
            let i2c_client = adxl_lock.client();
            // Set the `clientdata` to point to the `adxl345driver` instance
            // This will be freed automatically by remove callback (see i2c/driver.rs/remove_callback)
            i2c_client.set_clientdata::<Adxl345Driver>(unsafe{adxl345driver.as_mut().get_unchecked_mut()});
        }

        // Use I2CDriverBuilder to create and register the driver with probe and remove callbacks
        let driver_name = CStr::from_bytes_with_nul(DR_NAME_WN).unwrap().as_ptr() as *const i8;

        let builder = I2CDriverBuilder::<Adxl345Driver>::new(
            __I2C_DEVICE_TABLE_BINDINGS.as_ptr(),
            driver_name, 
            module.as_ptr(), 
        );

        // Build driver structure, then add it
        let driver = builder.build().expect("Failed I2C Driver build");

        driver.add_driver().expect("Failed when adding driver");
    
        // Store I2CDriver structure inside Adxl345Driver
        adxl345driver.as_mut().set_driver_pinned(driver);
        pr_info!("Adxl345 Driver correctly initialzied");

        Ok(Adxl345Module{the_driver: adxl345driver})
    }
}

impl Drop for Adxl345Module {
    fn drop(&mut self) {
        // Call `remove_driver` to unregister the driver
        self.the_driver.as_ref().driver().expect("Driver not initialized").remove_driver();

        // i2c client is dropped automatically by its own trait.
        pr_info!("Adxl345 driver unloaded\n");
    }
}