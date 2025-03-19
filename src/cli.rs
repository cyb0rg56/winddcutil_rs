use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "winddcutil_rs",
    about = "Windows implementation of the ddcutil Linux program for querying and changing monitor settings",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Detect monitors
    Detect,
    
    /// Query monitor capabilities
    Capabilities {
        /// Display ID
        display: u32,
    },
    
    /// Set VCP feature value
    Setvcp {
        /// Display ID
        display: u32,
        
        /// Feature code (decimal or hex with 0x prefix)
        feature_code: String,
        
        /// New value (decimal or hex with 0x prefix)
        new_value: String,
    },
    
    /// Get VCP feature value
    Getvcp {
        /// Display ID
        display: u32,
        
        /// Feature code (decimal or hex with 0x prefix)
        feature_code: String,
    },
} 