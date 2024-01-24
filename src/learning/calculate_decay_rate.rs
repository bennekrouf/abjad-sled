pub fn calculate_decay_rate(elapsed_days: i64, repetitions: Option<i32>) -> f32 {
    // Adjust decay rate based on the number of repetitions (if available)
    let repetitions_factor = match repetitions {
        Some(rep_count) if rep_count > 0 => 1.0 / (rep_count as f32),
        _ => 1.0, // Default factor if no repetitions or repetitions is 0
    };

    // Calculate the decay rate based on elapsed days and repetitions factor
    // Adjust these parameters as needed for your specific application
    let decay_rate = 1.0 - (elapsed_days as f32 * repetitions_factor * 0.01); 

    // Ensure the decay rate is within bounds (between 0 and 1)
    decay_rate.clamp(0.0, 1.0)
}
