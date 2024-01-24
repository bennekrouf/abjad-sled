pub fn scale_to_percentage(score: f32, min_value: f32, max_value: f32) -> f32 {
    // Ensure that the score is within the valid range
    let clamped_score = score.clamp(min_value, max_value);
    
    // Calculate the percentage based on the scaled score
    let percentage = ((clamped_score - min_value) / (max_value - min_value)) * 100.0;
    
    // Ensure that the percentage is within the [0, 100] range
    percentage.clamp(0.0, 100.0)
}
