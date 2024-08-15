use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GpuData {
    pub name: String,
    pub temp: f64,
    pub watt: f64,
}

pub fn get_gpu_data() -> Result<GpuData, String> {
    // Replace with the actual command to get GPU data
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=name,temperature.gpu,power.draw")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut parts = output_str.split(',');

    let name = parts.next().unwrap_or_default().trim().to_string();
    let temp = parts.next().unwrap_or_default().trim().parse::<f64>().map_err(|e| e.to_string())?;
    let watt = parts.next().unwrap_or_default().trim().parse::<f64>().map_err(|e| e.to_string())?;

    Ok(GpuData { name, temp, watt })
}