use std::process::Command;

pub struct RawHardwareInfo {
    pub motherboard_serial: String,
    pub cpu_id: String,
    pub system_uuid: String,
}

pub fn get_raw_hardware_info() -> Result<RawHardwareInfo, String> {
    // ioreg -rd1 -c IOPlatformExpertDevice
    let output = Command::new("ioreg")
        .args(["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
        .map_err(|e| format!("Failed to execute ioreg: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    let mut system_uuid = "unknown_uuid".to_string();
    let mut motherboard_serial = "unknown_motherboard".to_string();

    for line in output_str.lines() {
        if line.contains("\"IOPlatformUUID\"") {
            if let Some(uuid) = line.split('=').nth(1) {
                system_uuid = uuid.trim().trim_matches('"').to_string();
            }
        } else if line.contains("\"IOPlatformSerialNumber\"") {
            if let Some(serial) = line.split('=').nth(1) {
                motherboard_serial = serial.trim().trim_matches('"').to_string();
            }
        }
    }

    // sysctl -n machdep.cpu.brand_string
    let cpu_output = Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .map_err(|e| format!("Failed to execute sysctl: {}", e))?;

    let cpu_id = String::from_utf8_lossy(&cpu_output.stdout).trim().to_string();
    let cpu_id = if cpu_id.is_empty() { "unknown_cpu".to_string() } else { cpu_id };

    Ok(RawHardwareInfo {
        motherboard_serial,
        cpu_id,
        system_uuid,
    })
}
