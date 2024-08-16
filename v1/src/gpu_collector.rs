use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GpuData {
    pub name: String,
    pub temp: f64,
    pub watt: f64,
    pub fan_speed: f64,
}

pub fn get_gpu_data() -> Result<Vec<GpuData>, String> {
    // Run the nvidia-smi command to get GPU data for all GPUs
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=name,temperature.gpu,power.draw,fan.speed")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut gpus = Vec::new();

    for line in output_str.lines() {
        let mut parts = line.split(',');

        let name = parts.next().unwrap_or_default().trim().to_string();
        let temp = parts.next().unwrap_or_default().trim().parse::<f64>().map_err(|e| e.to_string())?;
        let watt = parts.next().unwrap_or_default().trim().parse::<f64>().map_err(|e| e.to_string())?;

        // Handle fan speed; default to 0.0 if N/A
        let fan_speed_str = parts.next().unwrap_or_default().trim();
        let fan_speed = if fan_speed_str == "[N/A]" {
            0.0
        } else {
            fan_speed_str.parse::<f64>().unwrap_or(0.0)
        };

        gpus.push(GpuData { name, temp, watt, fan_speed });
    }

    Ok(gpus)
}