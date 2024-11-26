# ADXL345 Driver Codebase Documentation

This directory contains the complete source code for the Rust-based Linux kernel driver for the **ADXL345 accelerometer**. <br />
The driver is designed to be **plug-and-play**, creating an I2C client for the ADXL345 device upon module load and registering a character device for user-space interaction.

---

## **Files Overview**

### **1. `adxl345_core.rs`**
- **Purpose**: Core implementation of the ADXL345 kernel module and driver.
- **Description**:
  - Defines the kernel module and handles its registration.
  - Implements the `probe` and `remove` functions to manage the lifecycle of the ADXL345 device.
  - Automatically creates an I2C client for the ADXL345 device upon initialization.
  - Ensures seamless integration with the Linux kernel and I2C subsystem.
- **Key Features**:
  - Plug-and-play functionality for a single ADXL345 instance.
  - Manages I2C communication and device initialization.

---

### **2. `fileops.rs`**
- **Purpose**: Implementation of file operations for the character device registered by the driver.
- **Description**:
  - Provides functionality to interact with the driver from user space.
  - Implements key operations:
    - **Open**: Sets up the character device for user-space interaction.
    - **Read**: Retrieves measurement data from the accelerometer.
    - **Release**: Handles cleanup when the character device is closed.
  - Bridges kernel-level driver functionality with user-space programs.
- **Key Features**:
  - Enables access to accelerometer measurements via the character device.

---

### **3. `utility.rs`**
- **Purpose**: Helper functions for device initialization and cleanup.
- **Description**:
  - Contains utility functions to:
    - Initialize the ADXL345 device during module or file operations.
    - Clean up resources during module removal or file release.
  - Ensures the ADXL345 device is always in a consistent state during its lifecycle.
- **Key Features**:
  - Reusable code for managing device setup and teardown.
  - Ensures robust handling of resource allocation and deallocation.

---

### **4. `structures.rs`**
- **Purpose**: Definition of core data structures used in the driver.
- **Description**:
  - Defines the following structures:
    - **`Adxl345Sample`**: Represents a single accelerometer measurement.
    - **`Adxl345`**: Encapsulates:
      - The I2C client associated with the ADXL345 device.
      - The registration information for the character device.
    - **`Adxl345Driver`**: Manages the ADXL345 I2C driver instance, including driver-specific data.
- **Key Features**:
  - Centralizes data related to the ADXL345 device and driver.
  - Simplifies driver development by providing structured representations.

---

### **5. `constant.rs`**
- **Purpose**: Defines constants and register values for the ADXL345 device.
- **Description**:
  - Contains important constants required for:
    - Device configuration.
    - Register operations.
    - Measurement processing.
  - Centralizes all constants to improve maintainability.
- **Key Features**:
  - Includes all necessary ADXL345 register values.
  - Provides a clear and organized reference for driver developers.

---

## **How It Works**

1. **Module Initialization**:
   - `adxl345_core.rs` initializes the module and creates an I2C client for the ADXL345.
   - A character device is registered for user-space interaction.

2. **User Interaction**:
   - The user interacts with the driver through the character device (file operations defined in `fileops.rs`).
   - Measurement data can be read via the `read` operation.

3. **Device Lifecycle Management**:
   - `utility.rs` ensures proper setup and teardown of the ADXL345 device during file and module operations.
   - Resources are allocated and deallocated safely to prevent leaks or undefined behavior.

4. **Data Handling**:
   - `structures.rs` provides the necessary structures to manage device state, I2C communication, and driver operations.

5. **Device Configuration**:
   - `constant.rs` provides all necessary constants to configure and interact with the ADXL345 device registers.

---

## **Usage**
- Compile and load the kernel module (`adxl345_core.rs`) to register the ADXL345 driver.
- Use the character device to interact with the ADXL345 from user space.
- Refer to the `adxl345_test` user-space program for examples of reading accelerometer data.

