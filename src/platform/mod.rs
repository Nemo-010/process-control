//! Everything the app asks the operating system, behind one interface.
//!
//! Only Linux is implemented: a task manager reads the process table, and
//! every system keeps that in its own way. The other platforms build and run
//! the window, which says plainly that it has nothing to show there, rather
//! than pretending.
//!
//! Nothing here is allowed on the drawing thread: a read of the process table
//! on a busy machine takes longer than a frame.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::desktop;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
mod unsupported;
#[cfg(not(target_os = "linux"))]
pub use unsupported::*;
