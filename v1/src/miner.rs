use std::process::{Command, Child};
use std::error::Error;
use crate::algo::AlgoConfig;

pub async fn start_mining(config: AlgoConfig) -> Result<Child, Box<dyn Error>> {
    let process = Command::new("lolMiner")
        .arg("--algo")
        .arg(config.algorithm)
        .arg("--pool")
        .arg(config.pool_url)
        .arg("--user")
        .arg(config.user)
        .arg("--log")
        .arg(config.log_file)
        .spawn()?;
    
    println!("Mining started with algorithm: {}", config.algorithm);
    Ok(process)
}

pub async fn stop_mining() -> Result<(), Box<dyn Error>> {
    Command::new("pkill")
        .arg("lolMiner")
        .spawn()?
        .wait()?;
    
    println!("Mining stopped.");
    Ok(())
}