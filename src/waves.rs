use rand::random;

/// Generates a sine wave from samples
pub fn sine_wave(t: f32) -> f32 {
    t.sin()
}

/// Generates a square wave from samples
pub fn square_wave(t: f32) -> f32 {
    t.sin().round()
}

/// Generates a saw-tooth wave from samples
pub fn saw_wave(t: f32) -> f32 {
    t - t.floor()
}

/// Generates white noise from samples
pub fn noise_wave(_: f32) -> f32 {
    (random::<f32>() * 2f32) - 1f32
}
