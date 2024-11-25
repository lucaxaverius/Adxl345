// i2c_macros.rs

//! Module for I2C-related macros.
//!
//! This module provides macros to assist in driver development, such as generating
//! device tables and callback functions.

/// Exposes the device table to the kernel module loader.
///
/// Is similar to the `MODULE_DEVICE_TABLE` macro in C with a few more parameters.
///
/// # Parameters
///
/// * `$type_` - The device type identifier (e.g., `i2c`).
/// * `$name` - The name of your device ID table variable.
/// * `$device_id_type` - The full path to the device ID type.
/// * `$len` - The length of your device ID table array.
///
/// # Notes
///
/// - The macro exports a symbol with a name in the format:
///   `__mod_<type>__<name>_device_table`
/// - Can be wrapped inside device_type specific macro, like i2c_module_device_table.
#[macro_export]
macro_rules! module_device_table {
    ($type_:ident, $name:ident, $device_id_type:path, $len:expr) => {
        #[no_mangle]
        #[link_section = ".modinfo"]
        #[export_name = concat!(
            "__mod_",
            stringify!($type_),
            "__",
            stringify!($name),
            "_device_table"
        )]
        /// The array exposed to modinfo
        pub static __DEVICE_TABLE_ALIAS: [$device_id_type; $len] = $name;
    };
}

/// Exposes the I2C device table to the kernel module loader.
///
/// Converts an array of `I2CDeviceID` to an array of `bindings::i2c_device_id`
/// and exports it for driver matching.
///
/// # Parameters
///
/// * `$name` - The name of your `I2CDeviceID` table.
/// * `$len` - The length of your `I2CDeviceID` table array.
///
/// # Notes
/// - The converted table is named: __I2C_DEVICE_TABLE_BINDINGS, 
///   it must be used when building a new driver with I2CDriverBuilder.
#[macro_export]
macro_rules! i2c_module_device_table {
    ($name:ident, $len:expr) => {
        /// The static array of bindings generated from the I2C device ID table.
        /// This array is used to expose the device table to the kernel module loader.
        static __I2C_DEVICE_TABLE_BINDINGS: [kernel::bindings::i2c_device_id; $len] =
            kernel::i2c::I2CDeviceID::to_bindings_array(&$name);

        // Expose the device table to the kernel module loader
        kernel::module_device_table!(
            i2c,
            __I2C_DEVICE_TABLE_BINDINGS,
            kernel::bindings::i2c_device_id,
            $len
        );
    };
}

