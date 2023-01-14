use rand::random;

/// Trait to represent possible wave functions
pub trait Wave: Sized {
    /// Calculates the wave
    fn calculate(&self, t: f32) -> f32;

    /// Mix waves using a specified ratio.
    fn mix<A: Wave, B: Wave>(self, a: A, b: B) -> MixWave<A, B, Self> {
        MixWave { a, b, ratio: self }
    }

    /// Add waves.
    fn add<W: Wave>(self, other: W) -> AddWave<Self, W> {
        AddWave { a: self, b: other }
    }

    /// Multiply waves.
    fn multiply<W: Wave>(self, other: W) -> MultiplyWave<Self, W> {
        MultiplyWave { a: self, b: other }
    }

    /// Reverse the wave.
    fn reverse(self) -> ReverseWave<Self> {
        ReverseWave(self)
    }

    /// Flip the wave.
    fn flip(self) -> FlipWave<Self> {
        FlipWave(self)
    }
}

impl Wave for fn(f32) -> f32 {
    fn calculate(&self, t: f32) -> f32 {
        self(t)
    }
}

impl Wave for f32 {
    #[inline(always)]
    fn calculate(&self, _t: f32) -> f32 {
        *self
    }
}

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

/// Mixes waves using a specified ratio.
pub struct MixWave<A: Wave, B: Wave, R: Wave> {
    a: A,
    b: B,
    ratio: R,
}

impl<A: Wave, B: Wave, R: Wave> Wave for MixWave<A, B, R> {
    fn calculate(&self, t: f32) -> f32 {
        let ratio = self.ratio.calculate(t) / 2.0 + 0.5;
        self.a.calculate(t) * ratio + self.b.calculate(t) * (1.0 - ratio)
    }
}

/// Adds waves.
pub struct AddWave<A: Wave, B: Wave> {
    a: A,
    b: B,
}

impl<A: Wave, B: Wave> Wave for AddWave<A, B> {
    fn calculate(&self, t: f32) -> f32 {
        self.a.calculate(t) + self.b.calculate(t)
    }
}

/// Multipies waves.
pub struct MultiplyWave<A: Wave, B: Wave> {
    a: A,
    b: B,
}

impl<A: Wave, B: Wave> Wave for MultiplyWave<A, B> {
    fn calculate(&self, t: f32) -> f32 {
        self.a.calculate(t) * self.b.calculate(t)
    }
}

/// Reverses a wave.
pub struct ReverseWave<W: Wave>(W);

impl<W: Wave> Wave for ReverseWave<W> {
    fn calculate(&self, t: f32) -> f32 {
        self.0.calculate(1.0 - t)
    }
}

/// Flips a wave.
pub struct FlipWave<W: Wave>(W);

impl<W: Wave> Wave for FlipWave<W> {
    fn calculate(&self, t: f32) -> f32 {
        -self.0.calculate(1.0 - t)
    }
}
