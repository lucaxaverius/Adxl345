# Rust I2C Abstraction for Linux Kernel

This module provides a Rust-based abstraction for the Linux Kernel's I2C subsystem, aiming to simplify the development of I2C drivers while maintaining high safety standards. The abstraction offers comprehensive support for creating and managing I2C drivers and clients, including communication via the SMBus protocol.

---

## **Key Features**

- **Safe and idiomatic Rust interfaces** for managing I2C drivers and clients.
- Provides abstractions for:
  - I2C drivers, clients, and adapters.
  - Board information and device IDs.
- Full support for callback management in Rust, including `probe`, `remove`, and `shutdown` functions.
- Wraps unsafe operations into safe, reusable Rust constructs, with flexibility for advanced users.

---

## **Files Overview**

### **1. `driver.rs`**
- **Purpose**: Provides the Rust abstraction for I2C drivers.
- **Description**:
  - Represents an I2C driver with the `I2CDriver` structure.
  - Includes methods to register and unregister an I2C driver.
  - Provides the **`I2CDriverBuilder`**, a utility structure that ensures safe creation of `I2CDriver` instances by managing fields of the underlying C struct.
  - Defines the **`I2CDriverCallbacks`** trait, which allows developers to implement driver callbacks (`probe`, `remove`, `shutdown`, etc.) entirely in Rust.
  - Wraps Rust callback functions into `unsafe extern "C"` functions compatible with the Linux Kernel's `i2c_driver`.

---

### **2. `client.rs`**
- **Purpose**: Represents I2C clients and facilitates communication with I2C slave devices.
- **Description**:
  - Implements the `I2CClient` structure, encapsulating the I2C client instance.
  - Provides methods for SMBus protocol operations, including:
    - `read_byte`, `write_byte`
    - `read_word`, `write_word`
    - Other register read/write operations.
  - Ensures safe memory management and thread-safe interaction with I2C client devices.

---

### **3. Additional Components**
- **`adapter.rs`**:
  - Provides a minimal abstraction for I2C adapters.
  - Includes methods to retrieve adapters by bus number and perform basic operations.

- **`board_info.rs`**:
  - Represents I2C board information, enabling device registration and configuration.

- **`device_id.rs`**:
  - Contains structures and utilities for managing I2C device IDs.
  - Supports static device ID tables for driver-device matching.

---

## **Usage**

### **Driver Development**
1. Define your driver callbacks by implementing the `I2CDriverCallbacks` trait.
2. Use the `I2CDriverBuilder` to create a properly initialized `I2CDriver` instance.
3. Register the driver with the kernel.

### **Client Interaction**
- Utilize the `I2CClient` structure to interact with I2C slave devices.
- Perform register read/write operations using SMBus methods.

---

## **Supported Kernel Version**
- The abstraction is compatible with Linux Kernel **6.3** and has been tested on ARM architectures (BeagleBone Black) thanks to this [branch](#https://github.com/Rust-for-Linux/linux/tree/rust) provided by the Rust for Linux community where Rust has supports for ARM 32 bit.


---

This module aims to bridge the gap between low-level kernel operations and Rust's safety guarantees, making I2C driver development more accessible and reliable.
