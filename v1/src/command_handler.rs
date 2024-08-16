// src/command_handler.rs

use std::process::Command;
use std::error::Error;

pub async fn start_mining(algo: &str, wallet_address: &str, pool_url: &str) -> Result<(), Box<dyn Error>> {
    Command::new("t-rex")
        .arg("-a")
        .arg(algo)
        .arg("-o")
        .arg(pool_url)
        .arg("-u")
        .arg(wallet_address)
        .arg("-p")
        .arg("x")
        .spawn()?
        .wait()?;

    println!("Mining started with algorithm: {}", algo);
    Ok(())
}

pub async fn stop_mining() -> Result<(), Box<dyn Error>> {
    Command::new("pkill")
        .arg("t-rex")
        .spawn()?
        .wait()?;

    println!("Mining stopped.");
    Ok(())
}

// Add more functions for overclocking, updating, etc. as needed.