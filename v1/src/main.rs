use dotenv::dotenv;
use tokio::task;
use tokio::time::{sleep, Duration, Instant};

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
                let start_time = Instant::now();

                if let Ok(gpu_data_list) = gpu_collector::get_gpu_data() {
                    for gpu_data in gpu_data_list {
                        // Log the GPU data being sent
                        println!("Sending GPU Data: {:?}", gpu_data);

                        // Send the GPU data over the WebSocket connection
                        websocket_handler::send_gpu_data(&mut websocket_conn, &gpu_data).await;
                    }
                } else {
                    eprintln!("Failed to get GPU data");
                }

                let elapsed_time = start_time.elapsed();
                println!("Data collection and sending took: {:.2?}", elapsed_time);

                // Adjust the sleep to ensure approximately 1 second between sends
                let sleep_duration = if elapsed_time < Duration::from_secs(1) {
                    Duration::from_secs(1/3) - elapsed_time
                } else {
                    Duration::from_secs(0)
                };

                sleep(sleep_duration).await;
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