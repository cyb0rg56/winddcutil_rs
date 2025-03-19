use crate::error::{Result, WinddcutilError};

/// Parses a string as either a decimal or hexadecimal integer
/// Hexadecimal values are expected to be prefixed with "0x"
pub fn parse_number(input: &str) -> Result<u16> {
    if input.starts_with("0x") || input.starts_with("0X") {
        match u16::from_str_radix(&input[2..], 16) {
            Ok(val) => Ok(val),
            Err(_) => Err(WinddcutilError::InvalidValue(format!("Invalid hexadecimal value: {}", input))),
        }
    } else {
        match input.parse::<u16>() {
            Ok(val) => Ok(val),
            Err(_) => Err(WinddcutilError::InvalidValue(format!("Invalid decimal value: {}", input))),
        }
    }
}

/// Formats a value as hexadecimal with 0x prefix
pub fn format_hex(value: u16) -> String {
    format!("0x{:02X}", value)
} 