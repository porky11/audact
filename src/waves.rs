use rand::random;

/// Trait to represent possible wave functions
pub trait Wave {
    /// Calculates the wave
    fn calculate(&self, t: f32) -> f32;

    /// Use a specific frequency for a wave.
    fn frequency(self, frequency: f32) -> FrequencyWave<Self>
    where
        Self: Sized,
    {
        FrequencyWave {
            wave: self,
            frequency,
        }
    }

    /// Add a specific octave for a wave.
    fn octave(self, octaves: f32) -> FrequencyWave<Self>
    where
        Self: Sized,
    {
        self.frequency(2.0f32.powf(octaves))
    }

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
#[derive(Clone, Copy)]
pub struct SineWave;

impl Wave for SineWave {
    fn calculate(&self, t: f32) -> f32 {
        ((t * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0
    }
}

/// A simple triangle wave.
#[derive(Clone, Copy)]
pub struct TriangleWave;

impl Wave for TriangleWave {
    fn calculate(&self, t: f32) -> f32 {
        ((t * 2.0) % 2.0 - 1.0).abs()
    }
}

/// A simple square wave.
#[derive(Clone, Copy)]
pub struct SquareWave;

impl Wave for SquareWave {
    fn calculate(&self, t: f32) -> f32 {
        if t % 1.0 < 0.5 {
            0.0
        } else {
            1.0
        }
    }
}

/// A simple saw wave.
#[derive(Clone, Copy)]
pub struct SawWave;

impl Wave for SawWave {
    fn calculate(&self, t: f32) -> f32 {
        t % 1.0
    }
}

/// A simple hill wave.
#[derive(Clone, Copy)]
pub struct HillWave;

impl Wave for HillWave {
    fn calculate(&self, t: f32) -> f32 {
        (t * std::f32::consts::PI).sin().abs()
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

/// Wave having a specific frequency.
#[derive(Clone, Copy)]
pub struct FrequencyWave<W: Wave> {
    wave: W,
    frequency: f32,
}

impl<W: Wave> Wave for FrequencyWave<W> {
    fn calculate(&self, t: f32) -> f32 {
        self.wave.calculate(t * self.frequency)
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
