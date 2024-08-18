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

    // Establish the WebSocket connection and start listening for commands
    let websocket_conn = websocket_handler::listen_for_commands().await;

    if let Some(mut websocket_conn) = websocket_conn {
        // Task for sending GPU data periodically
        task::spawn(async move {
            loop {
                if let Ok(gpu_data_list) = gpu_collector::get_gpu_data() {
                    for gpu_data in gpu_data_list {
                        println!("GPU Data: {:?}", gpu_data); // Optional: For local debug logging
                        // Send the GPU data over the WebSocket connection
                        websocket_handler::send_gpu_data(&mut websocket_conn, &gpu_data).await;
                    }
                } else {
                    eprintln!("Failed to get GPU data");
                }
                sleep(Duration::from_secs(10)).await;
            }
        });
    } else {
        eprintln!("Failed to establish WebSocket connection.");
    }

    // Keep the main function alive
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}