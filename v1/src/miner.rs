use std::process::Command;
use std::error::Error;

pub async fn start_ravencoin_mining() -> Result<(), Box<dyn Error>> {
    Command::new("t-rex")
        .arg("-a")
        .arg("kawpow")
        .arg("-o")
        .arg("stratum+tcp://rvn.2miners.com:6060")
        .arg("-u")
        .arg("your_wallet_address")
        .arg("-p")
        .arg("x")
        .spawn()?
        .wait()?;
    
    println!("Ravencoin mining started.");
    Ok(())
}

pub async fn stop_ravencoin_mining() -> Result<(), Box<dyn Error>> {
    Command::new("pkill")
        .arg("t-rex")
        .spawn()?
        .wait()?;
    
    println!("Ravencoin mining stopped.");
    Ok(())
}

// Add more functions for other algorithms as needed.