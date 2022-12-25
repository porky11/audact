use rand::random;

use crate::system::Wave;

/// A simple sine wave.
pub struct SineWave;

impl Wave for SineWave {
    fn calculate(&self, t: f32) -> f32 {
        (t * std::f32::consts::PI).sin().round()
    }
}

/// A simple square wave.
pub struct SquareWave;

impl Wave for SquareWave {
    fn calculate(&self, t: f32) -> f32 {
        if t % 1.0 < 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

/// A simple saw wave.
pub struct SawWave;

impl Wave for SawWave {
    fn calculate(&self, t: f32) -> f32 {
        (t % 1.0) * 2.0 - 1.0
    }
}

/// A simple noise wave.
pub struct NoiseWave;

impl Wave for NoiseWave {
    fn calculate(&self, _: f32) -> f32 {
        random::<f32>() * 2.0 - 1.0
    }
}
