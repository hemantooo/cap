use std::fs;

pub struct RawHardwareInfo {
    pub motherboard_serial: String,
    pub cpu_id: String,
    pub system_uuid: String,
}

pub fn get_raw_hardware_info() -> Result<RawHardwareInfo, String> {
    let system_uuid = fs::read_to_string("/sys/class/dmi/id/product_uuid")
        .or_else(|_| fs::read_to_string("/etc/machine-id"))
        .or_else(|_| fs::read_to_string("/var/lib/dbus/machine-id"))
        .unwrap_or_else(|_| "unknown_uuid".to_string())
        .trim()
        .to_string();

    let motherboard_serial = fs::read_to_string("/sys/class/dmi/id/product_serial")
        .unwrap_or_else(|_| "unknown_motherboard".to_string())
        .trim()
        .to_string();

    // Extract CPU model/ID from /proc/cpuinfo
    let cpu_info = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let cpu_id = cpu_info
        .lines()
        .find(|line| line.starts_with("model name") || line.starts_with("Hardware"))
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                parts[1].trim().to_string()
            } else {
                "unknown_cpu".to_string()
            }
        })
        .unwrap_or_else(|| "unknown_cpu".to_string());

    Ok(RawHardwareInfo {
        motherboard_serial,
        cpu_id,
        system_uuid,
    })
}
