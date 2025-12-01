use crate::error::{Result, WinddcutilError};
use crate::utils;
use ddc::Ddc;
use ddc_winapi::Monitor;
use display_info::DisplayInfo;

pub fn get_monitors() -> Result<Vec<Monitor>> {
    let monitors = Monitor::enumerate().map_err(|e| WinddcutilError::DdcError(e.to_string()))?;
    if monitors.is_empty() {
        return Err(WinddcutilError::NoMonitorsFound);
    }
    
    Ok(monitors)
}

pub fn get_display_info() -> Result<Vec<DisplayInfo>> {
    match DisplayInfo::all() {
        Ok(info) => Ok(info),
        Err(_) => Err(WinddcutilError::DisplayInfoError("Failed to get display information".to_string())),
    }
}

pub fn get_monitor_by_id(id: u32) -> Result<Monitor> {
    let monitors = get_monitors()?;
    
    if id < 1 || id > monitors.len() as u32 {
        return Err(WinddcutilError::MonitorNotFound(id));
    }
    
    // Return a copy of the monitor
    let idx = id as usize - 1;
    Ok(monitors.into_iter().nth(idx).unwrap())
}

pub fn detect_monitors() -> Result<()> {
    let monitors = get_monitors()?;
    let display_info = get_display_info()?;
    
    for (i, _) in monitors.iter().enumerate() {
        let index = i + 1;
        
        if let Some(info) = display_info.get(i) {
            println!(
                "{}: Monitor at ({},{}) - {} x {} - {}",
                index,
                info.x, info.y,
                info.width, info.height,
                info.id
            );
        } else {
            println!("{}: Unknown monitor", index);
        }
    }
    
    Ok(())
}

pub fn get_capabilities(monitor_id: u32) -> Result<()> {
    let mut monitor = get_monitor_by_id(monitor_id)?;
    
    println!("Monitor capabilities:");
    println!("Note: Full capability listing not available in this implementation");
    
    let features = [
        (0x10u8, "Brightness"),
        (0x12u8, "Contrast"),
        (0x60u8, "Input Source"),
        (0xD6u8, "Power Mode"),
    ];
    for &(feature_code, feature_name) in &features {
        match monitor.get_vcp_feature(feature_code) {
            Ok(value) => {
                println!("VCP 0x{:02X} ({}): Value={:?}", feature_code, feature_name, value);
            },
            Err(_) => {
                println!("VCP 0x{:02X} ({}): Not supported", feature_code, feature_name);
            }
        }
    }
    
    Ok(())
}

pub fn set_vcp_feature(monitor_id: u32, feature_code: u16, new_value: u16) -> Result<()> {
    let mut monitor = get_monitor_by_id(monitor_id)?;
    
    if feature_code > 255 {
        return Err(WinddcutilError::InvalidFeatureCode(format!("Feature code {} exceeds maximum value of 255", feature_code)));
    }
    
    match monitor.set_vcp_feature(feature_code as u8, new_value) {
        Ok(_) => Ok(()),
        Err(e) => Err(WinddcutilError::DdcError(e.to_string())),
    }
}


pub fn get_vcp_feature(monitor_id: u32, feature_code: u16) -> Result<()> {
    let mut monitor = get_monitor_by_id(monitor_id)?;
    
    if feature_code > 255 {
        return Err(WinddcutilError::InvalidFeatureCode(format!("Feature code {} exceeds maximum value of 255", feature_code)));
    }
    
    match monitor.get_vcp_feature(feature_code as u8) {
        Ok(value) => {
            println!("VCP {} {:?}", utils::format_hex(feature_code), value);
            Ok(())
        },
        Err(e) => Err(WinddcutilError::DdcError(e.to_string())),
    }
} 