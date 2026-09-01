use std::process::Command;

pub struct RawHardwareInfo {
    pub motherboard_serial: String,
    pub cpu_id: String,
    pub system_uuid: String,
}

fn execute_powershell(command: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", command])
        .output()
        .map_err(|e| format!("Failed to execute powershell: {}", e))?;
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_raw_hardware_info() -> Result<RawHardwareInfo, String> {
    let system_uuid = execute_powershell("(Get-CimInstance Win32_ComputerSystemProduct).UUID")?;
    let motherboard_serial = execute_powershell("(Get-CimInstance Win32_BaseBoard).SerialNumber")?;
    let cpu_id = execute_powershell("(Get-CimInstance Win32_Processor).ProcessorId")?;

    Ok(RawHardwareInfo {
        motherboard_serial: if motherboard_serial.is_empty() { "unknown_motherboard".to_string() } else { motherboard_serial },
        cpu_id: if cpu_id.is_empty() { "unknown_cpu".to_string() } else { cpu_id },
        system_uuid: if system_uuid.is_empty() { "unknown_uuid".to_string() } else { system_uuid },
    })
}
