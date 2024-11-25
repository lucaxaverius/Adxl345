# ADXL345 Rust Driver for BeagleBone Black

## Introduction
This repository showcases the development of a safe and efficient Rust-based Linux kernel driver for the ADXL345 accelerometer, specifically tailored for the BeagleBone Black. The driver communicates with the accelerometer using the I2C protocol and registers a character device to facilitate interaction from user space.

The project was built on top of a modified Linux kernel (version 6.3) with Rust support for ARM 32-bit, using the branch provided by the Rust for Linux [reository](#https://github.com/Rust-for-Linux/linux/tree/rust).

## Goals
The primary objectives of the project were:
1) **Create Safe Abstraction for I2C**:
    Develop robust and type-safe abstractions for the I2C subsystem in Rust, enabling interaction with I2C devices.
2) **Write the ADXL345 Driver in Rust**: Leverage these abstractions to implement a driver for the ADXL345 accelerometer.
3) **User-Space Testing**:  Provide a user-space program to interact with the driver and validate its functionality.

## Repository Structure
- **rust/**: Rust kernel source, includes the I2C Abastractions.
- **src/**: Source code for the ADXL345 Rust Driver.
- **adxl345_test/**: User-space test program that permits to interact with the driver.
- **add-dev.sh**: Script that adds the file associated to the char device.
- **.dts and .dtsi**: Device Tree Source file to enable I2C on Beaglebone Black 2014. 