//! GPU usage, from the kernel's own per-process accounting.
//!
//! Since kernel 6.4 an AMD or Intel GPU reports, for each open device file,
//! how long every engine has been busy. `/proc/<pid>/fdinfo/<fd>` carries that
//! as `drm-engine-<name>: <nanoseconds>`, and the device's total is the sum
//! of its engines' busy time, which `/sys/class/drm/cardN/device/gpu_busy_percent`
//! gives as a percentage.
//!
//! NVIDIA's driver keeps its own counters, which this does not read: the
//! column then stays empty rather than reporting a number that is not there.

use std::path::Path;

/// The GPU time and memory `directory`'s process holds.
///
/// Returns zeroes when the process has no GPU file open, which is every
/// process on a machine with no DRM accounting.
pub fn process(directory: &Path) -> (u64, u64) {
    let Ok(fdinfo) = std::fs::read_dir(directory.join("fdinfo")) else {
        return (0, 0);
    };
    let mut time = 0;
    let mut memory = 0;
    for entry in fdinfo.flatten() {
        let Ok(text) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        if !text.contains("drm-engine") {
            continue;
        }
        let (busy, used) = parse_fdinfo(&text);
        time += busy;
        memory = memory.max(used);
    }
    (time, memory)
}

/// The engine time and video memory a `fdinfo` file reports.
///
/// An engine line reads `drm-engine-render: 1234567890 ns`, and a memory line
/// `drm-memory-vram: 51200 KiB`. Anything else in the file is another
/// driver's business.
pub fn parse_fdinfo(text: &str) -> (u64, u64) {
    let mut time = 0;
    let mut memory = 0;
    for line in text.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if let Some(engine) = key.strip_prefix("drm-engine-") {
            let _ = engine;
            if let Some(nanoseconds) = value
                .strip_suffix(" ns")
                .and_then(|v| v.trim().parse::<u64>().ok())
            {
                time += nanoseconds;
            }
        } else if key.starts_with("drm-memory-")
            && key != "drm-memory"
            && let Some(bytes) = parse_size(value)
        {
            memory = memory.max(bytes);
        }
    }
    (time, memory)
}

/// A size as the kernel writes it, such as `51200 KiB`.
fn parse_size(value: &str) -> Option<u64> {
    let (number, unit) = value.split_once(' ')?;
    let number: u64 = number.trim().parse().ok()?;
    let scale = match unit.trim() {
        "B" => 1,
        "KiB" => 1024,
        "MiB" => 1024 * 1024,
        "GiB" => 1024 * 1024 * 1024,
        _ => return None,
    };
    number.checked_mul(scale)
}

/// The busy share of every GPU the machine reports, from 0 to 100.
///
/// `/sys/class/drm/cardN/device/gpu_busy_percent` is the kernel's own answer
/// for the whole device, so taking the busiest card is what the machine-wide
/// figure means when there is more than one.
pub fn busy_percent() -> Option<f32> {
    let Ok(entries) = std::fs::read_dir("/sys/class/drm") else {
        return None;
    };
    let mut busiest: Option<f32> = None;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        // `card0` alone; the `card0-DP-1` entries are connectors.
        if !name.starts_with("card") || name.contains('-') {
            continue;
        }
        let path = entry.path().join("device/gpu_busy_percent");
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let Ok(value) = text.trim().parse::<f32>() else {
            continue;
        };
        busiest = Some(busiest.map_or(value, |current: f32| current.max(value)));
    }
    busiest
}

/// The video memory in use, summed over every GPU the machine reports.
///
/// `/sys/class/drm/cardN/device/mem_info_vram_used` is the driver's own count.
pub fn memory_used() -> u64 {
    let Ok(entries) = std::fs::read_dir("/sys/class/drm") else {
        return 0;
    };
    let mut total = 0;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("card") || name.contains('-') {
            continue;
        }
        for file in ["mem_info_vram_used", "mem_info_gtt_used"] {
            let Ok(text) = std::fs::read_to_string(entry.path().join("device").join(file)) else {
                continue;
            };
            total += text.trim().parse::<u64>().unwrap_or(0);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fdinfo_file_is_read_into_time_and_memory() {
        let text = "\
pos:\t0
flags:\t02100002
mnt_id:\t123
drm-driver:\tamdgpu
drm-pdev:\t0000:03:00.0
drm-client-id:\t7
drm-engine-gfx:\t1234567890 ns
drm-engine-compute:\t100 ns
drm-memory-vram:\t51200 KiB
drm-memory-gtt:\t1024 KiB
drm-memory-cpu:\t8192 KiB
";
        let (time, memory) = parse_fdinfo(text);
        assert_eq!(time, 1_234_567_990);
        assert_eq!(memory, 51200 * 1024, "the largest region is the one used");
    }

    #[test]
    fn a_file_without_drm_counters_reads_as_nothing() {
        let (time, memory) = parse_fdinfo("pos:\t0\nflags:\t02100002\n");
        assert_eq!((time, memory), (0, 0));
    }

    #[test]
    fn sizes_are_read_in_their_units() {
        assert_eq!(parse_size("512 B"), Some(512));
        assert_eq!(parse_size("2 MiB"), Some(2 * 1024 * 1024));
        assert_eq!(parse_size("no number"), None);
        assert_eq!(parse_size("512 bananas"), None);
    }

    #[test]
    fn a_process_without_fdinfo_holds_no_gpu_time() {
        let missing = std::env::temp_dir().join("process-control-no-such-fdinfo");
        assert_eq!(process(&missing), (0, 0));
    }

    #[test]
    fn the_machine_reports_a_gpu_or_it_does_not() {
        // Either answer is fine; a percentage of a GPU that is there has to be
        // one a bar can draw.
        if let Some(busy) = busy_percent() {
            assert!((0.0..=100.0).contains(&busy), "busy is {busy}");
        }
    }
}
