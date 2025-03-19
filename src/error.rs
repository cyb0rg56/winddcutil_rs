use thiserror::Error;

#[derive(Error, Debug)]
pub enum WinddcutilError {
    #[error("No monitors with DDC/CI support found. Please ensure your monitor supports DDC/CI and it is enabled.")]
    NoMonitorsFound,
    
    #[error("Monitor with ID {0} not found")]
    MonitorNotFound(u32),
    
    #[error("Failed to access monitor: {0}")]
    MonitorAccessError(String),
    
    #[error("DDC/CI error: {0}")]
    DdcError(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    #[error("Invalid feature code: {0}")]
    InvalidFeatureCode(String),
    
    #[error("Display info error: {0}")]
    DisplayInfoError(String),
}

pub type Result<T> = std::result::Result<T, WinddcutilError>; 