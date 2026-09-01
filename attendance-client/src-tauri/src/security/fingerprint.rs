use sha2::{Sha256, Digest};
use super::platforms;

#[cfg(target_os = "linux")]
use platforms::linux::get_raw_hardware_info;

#[cfg(target_os = "macos")]
use platforms::macos::get_raw_hardware_info;

#[cfg(target_os = "windows")]
use platforms::windows::get_raw_hardware_info;

#[tauri::command]
pub fn get_device_fingerprint() -> Result<String, String> {
    let raw = get_raw_hardware_info()?;
    
    let canonical_str = format!("{}:{}:{}", raw.motherboard_serial, raw.cpu_id, raw.system_uuid);
    
    let mut hasher = Sha256::new();
    hasher.update(canonical_str.as_bytes());
    let result = hasher.finalize();
    
    Ok(hex::encode(result))
}
