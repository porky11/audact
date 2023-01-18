use rand::random;

/// Trait to represent possible wave functions
pub trait Wave {
    /// Calculates the wave
    fn calculate(&self, t: f32) -> f32;

    /// Mix waves using a specified ratio.
    fn mix<A: Wave, B: Wave>(self, a: A, b: B) -> MixWave<A, B, Self>
    where
        Self: Sized,
    {
        MixWave { a, b, ratio: self }
    }

    /// Add waves.
    fn add<W: Wave>(self, other: W) -> AddWave<Self, W>
    where
        Self: Sized,
    {
        AddWave { a: self, b: other }
    }

    /// Multiply waves.
    fn multiply<W: Wave>(self, other: W) -> MultiplyWave<Self, W>
    where
        Self: Sized,
    {
        MultiplyWave { a: self, b: other }
    }

    /// Reverse the wave.
    fn reverse(self) -> ReverseWave<Self>
    where
        Self: Sized,
    {
        ReverseWave(self)
    }

    /// Flip the wave.
    fn flip(self) -> FlipWave<Self>
    where
        Self: Sized,
    {
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

impl Wave for &dyn Wave {
    fn calculate(&self, t: f32) -> f32 {
        (*self).calculate(t)
    }
}

/// A simple sine wave.
#[derive(Default, Clone, Copy)]
pub struct SineWave(pub f32);

impl Wave for SineWave {
    fn calculate(&self, t: f32) -> f32 {
        ((t * 2.0f32.powf(self.0) * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0
    }
}

/// A simple square wave.
#[derive(Default, Clone, Copy)]
pub struct SquareWave(pub f32);

impl Wave for SquareWave {
    fn calculate(&self, t: f32) -> f32 {
        if (t * 2.0f32.powf(self.0)) % 1.0 < 0.5 {
            0.0
        } else {
            1.0
        }
    }
}

/// A simple saw wave.
#[derive(Default, Clone, Copy)]
pub struct SawWave(pub f32);

impl Wave for SawWave {
    fn calculate(&self, t: f32) -> f32 {
        (t * 2.0f32.powf(self.0)) % 1.0
    }
}

/// A simple noise wave.
#[derive(Clone, Copy)]
pub struct NoiseWave;

impl Wave for NoiseWave {
    fn calculate(&self, _: f32) -> f32 {
        random::<f32>()
    }
}

/// Mixes waves using a specified ratio.
#[derive(Clone, Copy)]
pub struct MixWave<A: Wave, B: Wave, R: Wave> {
    a: A,
    b: B,
    ratio: R,
}

impl<A: Wave, B: Wave, R: Wave> Wave for MixWave<A, B, R> {
    fn calculate(&self, t: f32) -> f32 {
        let ratio = self.ratio.calculate(t);
        self.a.calculate(t) * ratio + self.b.calculate(t) * (1.0 - ratio)
    }
}

/// Adds waves.
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct ReverseWave<W: Wave>(W);

impl<W: Wave> Wave for ReverseWave<W> {
    fn calculate(&self, t: f32) -> f32 {
        self.0.calculate(1.0 - t)
    }
}

/// Flips a wave.
#[derive(Clone, Copy)]
pub struct FlipWave<W: Wave>(W);

impl<W: Wave> Wave for FlipWave<W> {
    fn calculate(&self, t: f32) -> f32 {
        1.0 - self.0.calculate(t)
    }
}
