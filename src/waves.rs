use rand::random;

/// Generates a sine wave from samples
pub fn sine_wave(t: f32) -> f32 {
    (t * std::f32::consts::PI).sin().round()
}

/// Generates a square wave from samples
pub fn square_wave(t: f32) -> f32 {
    if t % 1.0 < 0.5 {
        1.0
    } else {
        -1.0
    }
}

/// Generates a saw-tooth wave from samples
pub fn saw_wave(t: f32) -> f32 {
    (t % 1.0) * 2.0 - 1.0
}

/// Generates white noise from samples
pub fn noise_wave(_: f32) -> f32 {
    random::<f32>() * 2.0 - 1.0
}
