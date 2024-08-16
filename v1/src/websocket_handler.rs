use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use crate::miner;

pub async fn listen_for_commands() {
    // Set up WebSocket connection to listen for commands
    let url = Url::parse("ws://localhost:9000").unwrap(); // Change this to your WebSocket server's URL
    let (mut websocket_conn, _) = connect_async(url).await.expect("Failed to connect to WebSocket");

    loop {
        match websocket_conn.next().await {
            Some(Ok(Message::Text(command))) => {
                handle_command(command).await;
            }
            Some(Err(e)) => {
                eprintln!("WebSocket error: {:?}", e);
            }
            _ => {}
        }
    }
}

async fn handle_command(command: String) {
    match command.as_str() {
        "start_ravencoin" => {
            if let Err(e) = miner::start_ravencoin_mining().await {
                eprintln!("Failed to start Ravencoin mining: {}", e);
            }
        }
        "stop_ravencoin" => {
            if let Err(e) = miner::stop_ravencoin_mining().await {
                eprintln!("Failed to stop Ravencoin mining: {}", e);
            }
        }
        // Add other commands here
        _ => {
            println!("Unknown command received: {}", command);
        }
    }
}