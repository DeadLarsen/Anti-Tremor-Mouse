use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, MouseState};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrationData {
    pub tremor_frequency: f64,    // Estimated tremor frequency in Hz
    pub tremor_amplitude: f64,    // Estimated tremor amplitude in pixels
    pub baseline_stability: f64,  // Baseline stability score
}

pub fn perform_calibration(device_state: &DeviceState) -> Result<CalibrationData> {
    println!("Please keep your mouse still for 5 seconds...");
    std::thread::sleep(Duration::from_secs(2));
    
    let mut positions = Vec::new();
    let start_time = Instant::now();
    
    // Collect mouse positions for 5 seconds
    while start_time.elapsed() < Duration::from_secs(5) {
        let mouse_state: MouseState = device_state.get_mouse();
        positions.push((mouse_state.coords.0 as f64, mouse_state.coords.1 as f64));
        std::thread::sleep(Duration::from_millis(10));
    }
    
    // Calculate tremor characteristics
    let (frequency, amplitude) = analyze_tremor(&positions);
    let stability = calculate_stability(&positions);
    
    Ok(CalibrationData {
        tremor_frequency: frequency,
        tremor_amplitude: amplitude,
        baseline_stability: stability,
    })
}

fn analyze_tremor(positions: &[(f64, f64)]) -> (f64, f64) {
    // Calculate average movement
    let mut total_distance = 0.0;
    for i in 1..positions.len() {
        let dx = positions[i].0 - positions[i-1].0;
        let dy = positions[i].1 - positions[i-1].1;
        total_distance += (dx * dx + dy * dy).sqrt();
    }
    
    // Estimate frequency based on zero crossings
    let mut zero_crossings = 0;
    for i in 1..positions.len() {
        let dx = positions[i].0 - positions[i-1].0;
        let dy = positions[i].1 - positions[i-1].1;
        let movement = (dx * dx + dy * dy).sqrt();
        
        if movement > 0.1 { // Threshold for significant movement
            zero_crossings += 1;
        }
    }
    
    let frequency = (zero_crossings as f64) / 5.0; // 5 seconds of data
    let amplitude = total_distance / (positions.len() as f64);
    
    (frequency, amplitude)
}

fn calculate_stability(positions: &[(f64, f64)]) -> f64 {
    let center_x = positions.iter().map(|p| p.0).sum::<f64>() / positions.len() as f64;
    let center_y = positions.iter().map(|p| p.1).sum::<f64>() / positions.len() as f64;
    
    let variance = positions.iter()
        .map(|(x, y)| {
            let dx = x - center_x;
            let dy = y - center_y;
            dx * dx + dy * dy
        })
        .sum::<f64>() / positions.len() as f64;
    
    1.0 / (1.0 + variance) // Convert to stability score between 0 and 1
} 