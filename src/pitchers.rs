use crate::waves::Wave;

/// Trait to represent possible pitch factors.
pub trait Pitcher {
    /// Calculates the pitch factor.
    fn calculate(&self, t: f32) -> f32;
}

impl Pitcher for fn(f32) -> f32 {
    fn calculate(&self, t: f32) -> f32 {
        self(t)
    }
}

impl Pitcher for f32 {
    #[inline(always)]
    fn calculate(&self, _t: f32) -> f32 {
        *self
    }
}

impl Pitcher for Vec<f32> {
    fn calculate(&self, t: f32) -> f32 {
        self[(t * self.len() as f32) as usize]
    }
}

impl Pitcher for &dyn Pitcher {
    fn calculate(&self, t: f32) -> f32 {
        (*self).calculate(t)
    }
}

/// A wave pitcher.
#[derive(Clone, Copy)]
pub struct WavePitcher<W: Wave> {
    wave: W,
    range: f32,
}

impl<W: Wave> Wave for WavePitcher<W> {
    fn calculate(&self, t: f32) -> f32 {
        let ratio = self.wave.calculate(t) * 2.0 - 1.0;
        2.0f32.powf(self.range * ratio)
    }
}
