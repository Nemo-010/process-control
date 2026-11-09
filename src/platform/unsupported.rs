//! The stand-in for the platforms this app has no reader for.
//!
//! Windows and macOS keep their process tables in forms whose readers are not
//! written yet. Everything here reports that plainly, so the window opens,
//! draws its own empty state and says why, rather than failing to build.

use std::time::{Duration, SystemTime};

use crate::sample::{ProcessDetails, ProcessInfo, Sample, Signal};

/// Why this platform has nothing to show.
pub const UNSUPPORTED: &str = "Reading the process table is implemented on Linux only.";

/// A reader that has no process table to read.
#[derive(Debug, Default)]
pub struct Reader;

impl Reader {
    /// A reader for a platform with no reader.
    pub fn new() -> Self {
        Self
    }

    /// Gives an empty sample that says why it is empty.
    pub fn read(&mut self) -> Sample {
        Sample {
            error: Some(UNSUPPORTED.to_string()),
            ..Sample::default()
        }
    }
}

/// Reads one process closely, which is nothing on this platform.
pub fn details(_pid: i32) -> Option<ProcessDetails> {
    None
}

/// Sends a signal, which this platform has no way to do yet.
pub fn signal(_pid: i32, _signal: Signal) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

/// Leaves the timer alone, since there is no sampler to keep precise.
pub fn keep_timers_fine() {}

/// The GPU's name, which this platform does not report here.
pub fn gpu_name() -> Option<String> {
    None
}

/// The name of the user with this id, which this platform does not report
/// here.
pub fn user_name(uid: u32) -> String {
    uid.to_string()
}

/// How long a process has been running, from the details it has.
pub fn uptime_of(details: &ProcessDetails) -> Option<Duration> {
    SystemTime::now().duration_since(details.started?).ok()
}
