use dotenv::dotenv;
use std::env;
use tokio::time::{sleep, Duration};

mod gpu_collector;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Main loop to monitor GPU data and send it to the central server
    loop {
        if let Err(e) = handlers::send_gpu_data().await {
            eprintln!("Failed to send GPU data: {}", e);
        }

        // Sleep for a predefined interval (e.g., 10 seconds) before sending the next data
        sleep(Duration::from_secs(10)).await;
    }
}

//does this work if i do this 