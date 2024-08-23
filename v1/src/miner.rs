use std::process::Command;
use std::error::Error;

pub async fn start_ergo_mining() -> Result<(), Box<dyn Error>> {
    Command::new("lolMiner")
        .arg("--algo")
        .arg("AUTOLYKOS2")
        .arg("--pool")
        .arg("stratum+tcp://de.ergo.herominers.com:1180")
        .arg("--user")
        .arg("9hwFm6uwUHT4vJUDg7KX8ucBtnPn817cEJrQ5qby292B9uQGWnN.MyWorker")  // Replace MyWorker with your worker name
        .arg("--log")
        .arg("logs/miner.log")
        .spawn()?
        .wait()?;
    
    println!("Ergo mining started.");
    Ok(())
}

pub async fn stop_mining() -> Result<(), Box<dyn Error>> {
    Command::new("pkill")
        .arg("lolMiner")
        .spawn()?
        .wait()?;
    
    println!("Mining stopped.");
    Ok(())
}