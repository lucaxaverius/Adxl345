#!/bin/bash

# Variables
DEVICE_NAME="adxl345"  # Replace with your device name
DEVICE_PATH="/dev/$DEVICE_NAME"
MINOR_NUMBER=0  # Replace with the desired minor number

# Check if the device name is provided in /proc/devices
MAJOR_NUMBER=$(grep "$DEVICE_NAME" /proc/devices | awk '{print $1}')

if [ -z "$MAJOR_NUMBER" ]; then
    echo "Error: Major number for device '$DEVICE_NAME' not found in /proc/devices."
    echo "Make sure the module is loaded and the device name matches."
    exit 1
fi

# Check if the device file already exists
if [ -e "$DEVICE_PATH" ]; then
    echo "Warning: Device file $DEVICE_PATH already exists. Removing it..."
    rm -f "$DEVICE_PATH"
fi

# Create the character device file
echo "Creating device file $DEVICE_PATH with major number $MAJOR_NUMBER and minor number $MINOR_NUMBER..."
mknod "$DEVICE_PATH" c "$MAJOR_NUMBER" "$MINOR_NUMBER"

# Check if the device file was created successfully
if [ -e "$DEVICE_PATH" ]; then
    echo "Device file $DEVICE_PATH created successfully."
else
    echo "Error: Failed to create device file $DEVICE_PATH."
    exit 1
fi

# Set permissions on the device file (optional)
chmod 666 "$DEVICE_PATH"
echo "Permissions set to 666 for $DEVICE_PATH."

# Success
echo "Device $DEVICE_NAME is ready for use."
