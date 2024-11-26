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


// utility.rs

use kernel::prelude::*;
use core::time::Duration;
use kernel::sync::{SpinLock, Arc};
use kernel::delay::coarse_sleep;
use kernel::error::Result;
use crate::structures::*;
use crate::constant::*;

/// Function that initializes an ADXL345 device with default configuration and performs a test read.
///
/// This function locks the provided `Spinlock<Adxl345>` as needed to manage concurrent access.
///
/// # Parameters
/// - `device`: A reference to the `Spinlock<Adxl345>` instance to initialize.
///
/// # Returns
/// - `Ok(())` if initialization is successful.
/// - `Err(Error)` if any I/O or configuration error occurs.
pub (crate) fn adxl345_device_init(device: Arc<SpinLock<Adxl345>>) -> Result<()> {

    {        
        // Acquire lock on the entire Adxl345 instance
        let adxl = device.lock();

        // Set default configuration
        adxl.set_default_config().map_err(|e| {
            pr_err!("Failed to set default configuration: error code {:?} \n",e);
            e
        })?;

        // Enable measurement for a data read test
        adxl.enable_measure().map_err(|e| {
            pr_err!("Failed to enable measurement: error code {:?} \n",e);
            e
        })?;

        // Release the lock temporarily for delay
    }
    
    // Unlocking before sleep
    coarse_sleep(Duration::from_millis(2)); 

    // Reacquire lock to perform data read
    let adxl = device.lock();

    // Perform a data read and log the sample
    match adxl.read_data() {
        Ok(sample) => {
            pr_info!(
                "x -> {}, y -> {}, z -> {} (mg)\n",
                sample.x, sample.y, sample.z
            );
        }
        Err(e) => {
            pr_info!("Failed to read data sample: {:?}\n",e);
            return Err(e);
        }
    }

    // Disable measurement mode
    adxl.disable_measure().map_err(|e| {
        pr_err!("Failed to disable measurement\n");
        e
    })

    // Here the spinlock guard will be automatically 
    // dropped cause goes out of scope
}



/// Function that cleans up the ADXL345 device by disabling interrupts and
/// setting it to standby mode.
///
/// This function locks the provided `Spinlock<Adxl345>` as needed to manage concurrent access.
///
/// # Parameters
/// - `device`: A reference to the `Spinlock<Adxl345>` instance to clean up.
///
/// # Returns
/// - `Ok(())` if the cleanup operations complete without errors.
/// - `Err(Error)` if any I/O error occurs during the cleanup process.
pub (crate) fn adxl345_device_clean(device: Arc<SpinLock<Adxl345>>) -> Result<()> {
    // Acquire lock on the entire Adxl345 instance
    let adxl = device.lock();

    // Disable device interrupts
    if let Err(e) = adxl.write_register(ADXL345_REG_INT_ENABLE, 0x00) {
        pr_err!("failed writing INT_ENABLE register\n");
        return Err(e);
    }

    // Put device in standby mode
    if let Err(e) = adxl.write_register(ADXL345_REG_POWER_CTL, 0x00) {
        pr_err!("failed writing POWER_CTL register\n");
        return Err(e);
    }

    // Lock is automatically dropped when `adxl` goes out of scope
    Ok(())
}

/// Function to initialize the ADXL345 device at file open time.
/// This enables measurement mode and waits for the device wake-up time.
///
/// This function locks the provided `Ref<Spinlock<Adxl345>>` as needed to manage concurrent access.
///
/// # Parameters
/// - `device`: A reference to the `Spinlock<Adxl345>` instance to initialize at open time.
///
/// # Returns
/// - `Ok(())` if the initialization is successful.
/// - `Err(Error)` if enabling measurement fails.
pub (crate) fn adxl345_device_init_at_open(device: Arc<SpinLock<Adxl345>>) -> Result<()> {
    // Acquire lock on the entire Adxl345 instance
    let adxl = device.lock();

    // Enable measurement mode
    let ret = adxl.enable_measure();
    
    // Release the lock before sleeping
    drop(adxl);

    // If enabling measurement was successful, wait for wake-up time
    if ret.is_ok() {
        coarse_sleep(Duration::from_millis(2)); // device wake-up time
    }

    ret
}


/// Function to clean up the ADXL345 device at file release time by disabling measurement mode.
///
/// This function locks the provided `Spinlock<Adxl345>` as needed to manage concurrent access.
///
/// # Parameters
/// - `device`: A reference to the `Spinlock<Adxl345>` instance to clean up at release time.
pub (crate) fn adxl345_device_clean_at_release(device: Arc<SpinLock<Adxl345>>) {
    // Acquire lock on the entire Adxl345 instance
    let adxl = device.lock();

    // Disable measurement mode
    let _ = adxl.disable_measure(); // Ignoring the result here as the original C code does

    // Lock is automatically dropped when `adxl` goes out of scope
}
