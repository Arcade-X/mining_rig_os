use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{StreamExt, SinkExt};
use futures_util::stream::SplitSink;
use url::Url;
use tokio::net::TcpStream;

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
        "start_ravencoin" => {
            if let Err(e) = crate::miner::start_ravencoin_mining().await {
                eprintln!("Failed to start Ravencoin mining: {}", e);
            }
        }
        "stop_ravencoin" => {
            if let Err(e) = crate::miner::stop_ravencoin_mining().await {
                eprintln!("Failed to stop Ravencoin mining: {}", e);
            }
        }
        _ => {
            println!("Unknown command received: {}", command);
        }
    }
}

pub async fn send_gpu_data(websocket_conn: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, gpu_data: &crate::gpu_collector::GpuData) {
    let data = serde_json::to_string(gpu_data).expect("Failed to serialize GPU data");
    if let Err(e) = websocket_conn.send(Message::Text(data)).await {
        eprintln!("Failed to send GPU data: {:?}", e);
    }
}