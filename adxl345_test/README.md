# ADXL345 Test Program Cross-Compilation Guide

This README explains how to cross-compile the ADXL345 test program for the BeagleBone Black, which uses an ARMv7 processor. It includes all necessary steps, from setting up the environment to transferring the compiled binary to the target device.

---

## Prerequisites

Ensure you have the following installed on your development machine (e.g., Ubuntu or Debian):

1. **Rust and Cargo** (Rust's package manager and build system):
   - Install Rust using `rustup`:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```
   - Add the Rust target for ARMv7:
     ```bash
     rustup target add armv7-unknown-linux-gnueabihf
     ```

2. **ARM Cross-Compiler**:
   - Install the ARM cross-compiler toolchain:
     ```bash
     sudo apt update
     sudo apt install gcc-arm-linux-gnueabihf libc6-dev-armhf-cross
     ```
---

## Project Setup

1. Clone or navigate to your Rust project directory containing the test code:
   ```bash
   cd /path/to/adxl345_test

2. Create or update the `.cargo/config.toml` file to configure Cargo for cross-compilation: 
    ```bash
    mkdir -p .cargo
    nano .cargo/config.toml
    ```

    Add this inside the config file:
    ```bash
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    ```

## Build and test the program

1. Compile the code for ARMv7 target:
    ```bash
    cargo build --target armv7-unknown-linux-gnueabihf --release
    ```
    The output will be located at:
    ```bash
    target/armv7-unknown-linux-gnueabihf/release/adxl345_test
    ```
