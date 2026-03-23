//! SuperInstance Onboarding CLI
//! 
//! Run this to set up your SuperInstance Ranch for the first time.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing::info;

mod onboarding;

use onboarding::run_wizard;

#[derive(Parser, Debug)]
#[command(name = "superinstance-onboard")]
#[command(about = "SuperInstance Onboarding Wizard", long_about = None)]
struct Args {
    /// Path to state file
    #[arg(short, long, default_value = "pasture/onboarding_state.json")]
    state: String,
    
    /// Force reconfiguration even if already set up
    #[arg(short, long)]
    reconfigure: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    let state_path = PathBuf::from(&args.state);
    
    // If reconfiguring, delete existing state
    if args.reconfigure && state_path.exists() {
        info!("Reconfiguring - removing existing state");
        std::fs::remove_file(&state_path)?;
    }
    
    // Run the wizard
    run_wizard(state_path).await?;
    
    Ok(())
}
