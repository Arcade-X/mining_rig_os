use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{StreamExt, SinkExt};
use futures_util::stream::SplitSink;
use url::Url;
use tokio::net::TcpStream;
use crate::gpu_collector::GpuData;
use crate::algo::{get_ergo_config};
use crate::miner::start_mining;

pub async fn listen_for_commands() -> Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>> {
    // Set up WebSocket connection to listen for commands
    let url = Url::parse("ws://192.168.178.10:8080/ws/").unwrap(); // Replace with your Mac's IP address
    let (websocket_conn, _) = connect_async(url).await.expect("Failed to connect to WebSocket");

    // Split the connection into write and read halves
    let (write, mut read) = websocket_conn.split();

    // Spawn a task to listen for incoming commands
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(command))) = read.next().await {
            handle_command(command).await;
        }
    });

    // Return the write half of the connection for sending messages
    Some(write)
}

async fn handle_command(command: String) {
    match command.as_str() {
        "start_ergo" => {
            let config = get_ergo_config();
            if let Err(e) = start_mining(config).await {
                eprintln!("Failed to start Ergo mining: {}", e);
            }
        }
        "stop_mining" => {
            if let Err(e) = crate::miner::stop_mining().await {
                eprintln!("Failed to stop mining: {}", e);
            }
        }
        _ => {
            println!("Unknown command received: {}", command);
        }
    }
}

pub async fn send_gpu_data(websocket_conn: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, gpu_data: &GpuData) {
    let data = serde_json::to_string(gpu_data).expect("Failed to serialize GPU data");
    if let Err(e) = websocket_conn.send(Message::Text(data)).await {
        eprintln!("Failed to send GPU data: {:?}", e);
    }
}