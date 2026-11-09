//! Everything the app asks the operating system.
//!
//! Only Linux is implemented, and only Linux is built: a task manager reads
//! the process table, and every system keeps that in its own way. Rather than
//! carry a stub for platforms the app cannot serve, the rest of them fail to
//! build with a message that says why.
//!
//! Nothing here is allowed on the drawing thread: a read of the process table
//! on a busy machine takes longer than a frame.

#[cfg(not(target_os = "linux"))]
compile_error!(
    "Process Control reads the Linux process table: /proc, /sys and the X display. \
     It runs on Linux only."
);

mod linux;
pub use linux::*;
