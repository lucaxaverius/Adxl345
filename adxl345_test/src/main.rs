use std::env;
use std::io::{self};
use std::os::unix::io::{FromRawFd,AsRawFd};
use std::process::exit;
use std::mem;
use libc::{open, read, O_RDONLY};
#[repr(C)]
#[derive(Debug)]
struct Adxl345Sample {
    x: i16,
    y: i16,
    z: i16,
}

const BUFLEN: usize = 16;

fn main() -> io::Result<()> {
    // Check for the device file argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <device file>", args[0]);
        exit(1);
    }

    let file_path = &args[1];
    let c_file_path = std::ffi::CString::new(file_path.as_str()).unwrap();

    // Open the device file using libc::open
    let fd = unsafe { open(c_file_path.as_ptr(), O_RDONLY) };
    if fd < 0 {
        eprintln!("Failed to open {}: {}", file_path, io::Error::last_os_error());
        exit(1);
    }

    // SAFETY: Wrap the raw fd in a File to ensure proper closure when dropped
    let file = unsafe { std::fs::File::from_raw_fd(fd) };

    // Define buffer for reading data
    let mut buf: [Adxl345Sample; BUFLEN] = unsafe { mem::zeroed() };

    loop {
        // Attempt to read data from the device
        let ret = unsafe {
            read(
                file.as_raw_fd(),
                buf.as_mut_ptr() as *mut libc::c_void,
                mem::size_of::<Adxl345Sample>() * BUFLEN as libc::size_t,
            )
        };

        // Check for read errors
        if ret == -1 {
            eprintln!("Failed to read from device: {}", io::Error::last_os_error());
            exit(1);
        }

        // Ensure read result is aligned with sample size
        if ret as usize % mem::size_of::<Adxl345Sample>() != 0 {
            eprintln!("Unexpected read size: {}", ret);
            exit(1);
        }

        // Process each sample in the buffer
        let samples_read = ret as usize / mem::size_of::<Adxl345Sample>();
        for sample in &buf[..samples_read] {
            println!("x -> {:6}, y -> {:6}, z -> {:6} (mg)", sample.x, sample.y, sample.z);
        }
    }
}