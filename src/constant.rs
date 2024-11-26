//! Constants used in the ADXL345 driver.
//!
//! This module contains all register addresses, device settings, and configuration
//! parameters for the ADXL345 accelerometer, which are used throughout the driver.


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


// constants.rs

/// Driver name for the ADXL345 accelerometer.
#[allow(dead_code)]
pub (crate) const DR_NAME: &[u8] = b"adxl345";

#[allow(dead_code)]
pub (crate) const DR_NAME_WN: &[u8] = b"adxl345\0";


#[allow(dead_code)]
pub (crate) const ADXL345_I2C_ADAPTER: i32 = 1;

#[allow(dead_code)]
pub (crate) const ADXL345_I2C_ADDR: u16 = 0x1D;

// Fixed device ID code
#[allow(dead_code)]
pub (crate) const ADXL345_DEVID: u8 = 0xE5;

// Register addresses
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DEVID: u8 = 0x00;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_THRESH_TAP: u8 = 0x1D;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_OFSX: u8 = 0x1E;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_OFSY: u8 = 0x1F;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_OFSZ: u8 = 0x20;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DUR: u8 = 0x21;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_LATENT: u8 = 0x22;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_WINDOW: u8 = 0x23;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_THRES_ACT: u8 = 0x24;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_THRES_INACT: u8 = 0x25;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_TIME_INACT: u8 = 0x26;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_ACT_INACT_CTL: u8 = 0x27;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_THRES_FF: u8 = 0x28;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_TIME_FF: u8 = 0x29;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_TAP_AXES: u8 = 0x2A;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_ACT_TAP_STATUS: u8 = 0x2B;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_BW_RATE: u8 = 0x2C;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_POWER_CTL: u8 = 0x2D;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_INT_ENABLE: u8 = 0x2E;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_INT_MAP: u8 = 0x2F;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_INT_SOURCE: u8 = 0x30;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATA_FORMAT: u8 = 0x31;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAX0: u8 = 0x32;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAX1: u8 = 0x33;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAY0: u8 = 0x34;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAY1: u8 = 0x35;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAZ0: u8 = 0x36;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_DATAZ1: u8 = 0x37;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_FIFO_CTL: u8 = 0x38;
#[allow(dead_code)]
pub (crate) const ADXL345_REG_FIFO_STATUS: u8 = 0x39;