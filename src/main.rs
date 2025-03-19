mod cli;
mod error;
mod monitor;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect => {
            monitor::detect_monitors()?;
        }
        
        Commands::Capabilities { display } => {
            monitor::get_capabilities(display)?;
        }
        
        Commands::Setvcp { display, feature_code, new_value } => {
            let feature_code = utils::parse_number(&feature_code)?;
            let new_value = utils::parse_number(&new_value)?;
            
            monitor::set_vcp_feature(display, feature_code, new_value)?;
        }
        
        Commands::Getvcp { display, feature_code } => {
            let feature_code = utils::parse_number(&feature_code)?;
            
            monitor::get_vcp_feature(display, feature_code)?;
        }
    }
    
    Ok(())
}
