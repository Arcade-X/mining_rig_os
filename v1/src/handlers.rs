// src/handlers.rs

use crate::gpu_collector::get_gpu_data;

pub async fn send_gpu_data() -> Result<(), String> {
    let gpu_data_list = get_gpu_data().map_err(|e| {
        eprintln!("Failed to get GPU data: {}", e);
        e.to_string()
    })?;

    for gpu_data in gpu_data_list {
        println!("GPU Data: {:?}", gpu_data);
    }

    // If you later want to send this data, you'd use the reqwest client here

    Ok(())
}