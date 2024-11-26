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


// fileops.rs


use kernel::prelude::*;
use kernel::sync::{Mutex, SpinLock, Arc};
use kernel::file::{File, Operations};
use kernel::file::flags::*;
use kernel::chrdev::{Registration};
use kernel::error::{Result};
use kernel::error::code::{EINVAL, EAGAIN, EIO};
use kernel::ForeignOwnable;
use core::time::Duration;
use crate::structures::{Adxl345Sample, Adxl345};
use crate::utility::{adxl345_device_init_at_open,adxl345_device_clean_at_release};
use kernel::delay::coarse_sleep;
use kernel::io_buffer::IoBufferWriter;
use kernel::{mutex_init};


///  Global variable to hold the last measurement, protected by a mutex
static mut ADXL345_LAST_SAMPLE: Mutex<Adxl345Sample> = unsafe{Mutex::new(Adxl345Sample::new(0, 0, 0))};
pub(crate) static mut DEVICE_PTR: Option<Arc<SpinLock<Adxl345>>> = None;

/// Minimum change required to capture acceleration on any axis.
/// This constant defines the threshold for filtering out small changes in acceleration
/// to prevent capturing insignificant movements or noise. 
const ADXL345_FILTER: i16 = 50;

/// Check on all the axys if the movement is greater than the minimun designed to take the sample.
fn adxl345_filter_out(new_sample: &Adxl345Sample) -> bool {
    // Lock the global filter state to read and update the last sample
    let mut last_sample = unsafe{ADXL345_LAST_SAMPLE.lock()};

    // Calculate absolute differences for x, y, and z axes
    let diff_x = (new_sample.x - last_sample.x).abs();
    if diff_x > ADXL345_FILTER {
        *last_sample = *new_sample; // Update last sample
        return false;
    }

    let diff_y = (new_sample.y - last_sample.y).abs();
    if diff_y > ADXL345_FILTER {
        *last_sample = *new_sample; // Update last sample
        return false;
    }

    let diff_z = (new_sample.z - last_sample.z).abs();
    if diff_z > ADXL345_FILTER {
        *last_sample = *new_sample; // Update last sample
        return false;
    }

    // Update last sample and return true if all diffs are within the threshold
    *last_sample = *new_sample;
    true
}



pub (crate) struct Adxl345FileOps {
}
// Mandatory by design, see file.rs/operations
unsafe impl Send for Adxl345FileOps{}
unsafe impl Sync for Adxl345FileOps{}

impl Operations for Adxl345FileOps {
    type Data: = ();
    type OpenData = ();

    const HAS_READ: bool = true;
    // Required constant to indicate that the vtable should be used
    const USE_VTABLE_ATTR: () = ();

    // Open the char device, can't be open in write mode
    fn open(_context: &Self::OpenData, file: &File) -> Result<Self::Data> {

        // Check if the file was opened with write access and deny it if so
        let access_mode = file.flags() & O_ACCMODE;
        if access_mode == O_WRONLY || access_mode == O_RDWR {
            return Err(EPERM);
        }
        
        {
            // Access the global pointer
            let device = unsafe {
                DEVICE_PTR.as_ref().expect("Driver not initialized").clone()
            };
            // Initialize at open, enabling measurement mode
            adxl345_device_init_at_open(device).map_err(|_| EIO)?;
        }

        //Initialize the global Mutex.
        mutex_init!(unsafe { Pin::new_unchecked(&mut ADXL345_LAST_SAMPLE)}, "adxl345_last_sample");

        // Reset the global filter state
        let mut filter_last = unsafe{ADXL345_LAST_SAMPLE.lock()};
        *filter_last = Adxl345Sample { x: 0, y: 0, z: 0 };

        // Private data are automatically set to point to `dev`, see open_callback in file.rs
        
        // Set file as non-seekable
        file.set_nonseekable().expect("Can't set file as not seekeable");

        pr_info!("File open correctly executed \n");

        // Return a reference counted pointer of device
        Ok(())
    }

    /// Calls device clean at release and frees private date inside the file pointer
    fn release(_data: Self::Data, _file: &File){
        
        {    
             // Access the global pointer
            let device = unsafe {
                DEVICE_PTR.as_ref().expect("Driver not initialized").clone()
            };

            // Clean up at release (disable measurements)
            adxl345_device_clean_at_release(device);
        }

        // Private data are automatically set to null`, see release_callback in file.rs
    }

    /// Reads accelerometer data into the user's buffer, ensuring only one process reads at a time.
    fn read(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>, // Use ArcBorrow<'_, SpinLock<Adxl345>>        
        file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        
        let mut count = 0;

        {
            // Access the global pointer
            let device = unsafe {
                DEVICE_PTR.as_ref().expect("Driver not initialized").clone()
            };

            // Lock the entire `Adxl345` instance
            let adxl = device.lock();

            // Calculate the number of items based on the size of `Adxl345Sample`.
            let items = writer.len() / core::mem::size_of::<Adxl345Sample>();
            if items == 0 {
                return Err(EINVAL);
            }

            // Wait until data is ready or handle non-blocking mode.
            loop {
                // Check if data is ready
                match adxl.data_ready() {
                    Ok(ready) if ready > 0 => break,
                    /* data_ready == 0 and flags  */
                    Ok(_) if file.flags() & O_NONBLOCK != 0 => {
                        /* O_NONBLOCK == O_NDELAY */
                        return Err(EAGAIN);
                    }
                    // just sleep
                    Ok(_) => coarse_sleep(Duration::from_millis(10)),
                    // return error
                    Err(_) => return Err(EIO),
                }
            }

            // Begin reading measurements until the buffer is full.
            // for 0 .. items ensure that the loop stops when the space on the buffer ends.
            for _ in 0..items {
                // Read measurement data
                let acc = match adxl.read_data() {
                    Ok(sample) => sample,
                    Err(_) => return Err(EIO),
                };

                // Apply filtering: discard the misuration if the changes are to small
                if adxl345_filter_out(&acc) {
                    continue;
                }

                // Attempt to write each field to the user buffer, checking for errors on each operation
                if let Err(e) = writer.write(&acc.x) {
                    pr_err!("Failed to write X-axis data to user buffer: {:?}", e);
                    return Err(e);
                }

                if let Err(e) = writer.write(&acc.y) {
                    pr_err!("Failed to write Y-axis data to user buffer: {:?}", e);
                    return Err(e);
                }

                if let Err(e) = writer.write(&acc.z) {
                    pr_err!("Failed to write Z-axis data to user buffer: {:?}", e);
                    return Err(e);
                }
            

                count += core::mem::size_of::<Adxl345Sample>();

                // Check if more data is ready
                match adxl.data_ready() {
                    Ok(ready) if ready == 0 => break,
                    Ok(_) => continue,
                    Err(_) => return Err(EIO),
                }
            }
        }

        Ok(count)
    }
    
}

/// Registers a character device for the ADXL345 accelerometer.
///
/// This function registers a new character device in the system, making it available
/// under the specified name and minor number. Once registered, the device will be
/// automatically deregistered when the `Registration` instance is dropped, so there is no need to
/// call a separate deletion function.
///
/// # Arguments
/// - `name`: The device name, typically as a `CStr`.
/// - `minors_start`: The starting minor number for the device.
/// - `module`: A reference to the current module (usually `THIS_MODULE`).
///
/// # Returns
/// - `Result<Arc<Mutex<Registration<1>>>>`: An `Arc` containing the `Registration` object if
///   the registration is successful; otherwise, an error.
///
/// # Safety
/// This function uses kernel mechanisms for character device registration.
pub (crate) fn adxl345_chardev_add(
    name: &'static CStr,
    minors_start: u16,
    module: &'static kernel::ThisModule,
) -> Result<Pin<Box<Registration<1>>>> {
    // Create a new pinned `Registration` object for the character device
    let mut registration = Registration::new_pinned(name, minors_start, module)?;
    
    registration.as_mut().register::<Adxl345FileOps>().expect("Registration failed");

    Ok(registration)
}
