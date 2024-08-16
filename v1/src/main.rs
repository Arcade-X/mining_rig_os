use dotenv::dotenv;
use tokio::task;
use tokio::time::{sleep, Duration};

mod gpu_collector;
mod handlers;
mod websocket_handler;
mod miner;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Task for monitoring GPU data
    task::spawn(async {
        loop {
            if let Err(e) = handlers::send_gpu_data().await {
                eprintln!("Failed to send GPU data: {}", e);
            }
            sleep(Duration::from_secs(10)).await;
        }
    });

    // Task for listening to WebSocket commands
    task::spawn(async {
        websocket_handler::listen_for_commands().await;
    });

    // Keep the main function alive
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}