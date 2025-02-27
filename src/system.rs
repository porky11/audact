use crate::{pitchers::Pitcher, waves::Wave};

use std::{collections::VecDeque, num::NonZeroUsize, time::Duration};

use derive_builder::Builder;
use rodio::{
    buffer::SamplesBuffer, source, OutputStream, OutputStreamHandle, PlayError, Sink, Source,
    StreamError,
};

/// Struct for the main audact system
pub struct Audact {
    /// The output stream that will actually be played through
    /// kept here as won't work if it is dropped
    _output_stream: OutputStream,
    /// The output stream handle that audact will play through
    output_stream_handle: OutputStreamHandle,
    /// Vec of voice channels that audact will play
    channels: Vec<Channel>,
    /// Keep alive these sinks while playing
    sinks: VecDeque<Sink>,
    /// Maximum number of sinks
    max_sinks: Option<NonZeroUsize>,
    /// Sample rate
    sample_rate: u32,
}

/// Stuct to represent a channel
struct Channel {
    /// The Sink that the channel plays from
    sink: Sink,
    /// The samples the channel plays
    source: Vec<f32>,
    /// Processing for the channel
    processing: Processing,
}

impl Channel {
    fn new(
        audact: &Audact,
        wave: impl Wave,
        volume: impl Wave,
        processing: Processing,
        pitcher: impl Pitcher,
        bpm_duration: Duration,
    ) -> Result<Self, PlayError> {
        // create the sink to play from
        let sink = Sink::try_new(&audact.output_stream_handle)?;
        sink.pause();

        let sample_rate = audact.sample_rate as f32;
        let samples_needed = sample_rate * bpm_duration.as_secs_f32();

        // Create the basic waveform samples
        let mut source: Vec<f32> = (0u64..samples_needed as u64)
            .map(move |t| {
                let t = t as f32;
                let n_t = t / samples_needed;

                let freq = pitcher.calculate(n_t);

                // Silence if not playing in this step
                if freq == 0.0 {
                    return 0.0;
                }

                let t_f = t * freq / sample_rate;

                (wave.calculate(t_f) * 2.0 - 1.0) * volume.calculate(n_t)
            })
            .collect();

        Audact::smooth_source(&mut source);

        // Create the processing chain and channel
        Ok(Self {
            sink,
            source,
            processing,
        })
    }

    fn play(&self, sample_rate: u32, bars: i32) {
        for _ in 0..bars {
            // create buffer
            let samples = self.source.clone();
            let sample_buffer = vec![SamplesBuffer::new(2, sample_rate, samples)];
            // create the source
            let source = source::from_iter(sample_buffer)
                .buffered()
                .fade_in(self.processing.attack)
                .low_pass(self.processing.filter.1 as u32)
                .reverb(self.processing.reverb.0, self.processing.reverb.1)
                .amplify(self.processing.gain);
            // add source to sink queue
            self.sink.append(source);
        }

        self.sink.play();
    }
}

/// Represents processing values on a channel
#[derive(Builder, Clone, Copy)]
#[builder(default)]
pub struct Processing {
    /// Volume
    gain: f32,
    /// Filter
    filter: (f32, f32),
    /// Attack
    attack: Duration,
    /// Reverb
    reverb: (Duration, f32),
}

impl Default for Processing {
    fn default() -> Self {
        Processing {
            gain: 1f32,
            filter: (0f32, 5000f32),
            attack: Duration::from_millis(0u64),
            reverb: (Duration::from_millis(0), 0f32),
        }
    }
}

/// implementation for the audact struct
impl Audact {
    /// Creates a new instance of audact
    pub fn new() -> Result<Self, StreamError> {
        let (_output_stream, output_stream_handle) = OutputStream::try_default()?;
        let sample_rate = 44100f32;

        Ok(Self {
            _output_stream,
            output_stream_handle,
            channels: Vec::new(),
            sinks: VecDeque::new(),
            max_sinks: None,
            sample_rate: sample_rate as u32,
        })
    }

    /// Creates a new instance of audact with max sinks
    pub fn with_max_sinks(self, max_sinks: NonZeroUsize) -> Self {
        Self {
            max_sinks: Some(max_sinks),
            ..self
        }
    }

    /// Smooth out the generated source
    fn smooth_source(source: &mut Vec<f32>) {
        // Get rid of clicks by interpolating vol changes
        // probably smarter ways of doing this
        fn smooth(source: &mut [f32]) {
            let mut prev_sample = 0f32;
            for s in source.iter_mut() {
                if *s == 0f32 {
                    *s = prev_sample * 0.99;
                }
                prev_sample = *s;
            }
        }
        // smooth note off
        // then reverse to smooth note on then return to original
        for _ in 0..2 {
            let mut pad = vec![0f32; 1024usize];
            source.append(&mut pad);
            smooth(source);
            source.reverse();
        }
    }

    /// Add a voice channel to audact for synth playback
    pub fn channel(
        &mut self,
        wave: impl Wave,
        volume: impl Wave,
        processing: Processing,
        pitcher: impl Pitcher,
        bpm_duration: Duration,
    ) -> Result<(), PlayError> {
        let channel = Channel::new(self, wave, volume, processing, pitcher, bpm_duration)?;
        self.channels.push(channel);

        Ok(())
    }

    /// Start playing the audio `bars` times
    pub fn start(&self, bars: i32) {
        let sample_rate = self.sample_rate;
        for chan in &self.channels {
            chan.play(sample_rate, bars);
        }
    }

    /// Play single wave on a new channel
    pub fn play(
        &mut self,
        wave: impl Wave,
        volume: impl Wave,
        processing: Processing,
        pitcher: impl Pitcher,
        bpm_duration: Duration,
    ) -> Result<(), PlayError> {
        if let Some(max_sinks) = self.max_sinks {
            while self.sinks.len() > max_sinks.into() {
                self.sinks.pop_front();
            }
        }

        let channel = Channel::new(self, wave, volume, processing, pitcher, bpm_duration)?;
        channel.play(self.sample_rate, 1);
        self.sinks.push_back(channel.sink);

        Ok(())
    }

    /// Wait for the audio to end
    pub fn wait(&self) {
        // Sleep until the end of the sequence
        for chan in &self.channels {
            chan.sink.sleep_until_end();
        }
    }

    /// Play the audio `bars` times and wait for it to end
    pub fn run(&self, bars: i32) {
        self.start(bars);
        self.wait();
    }
}
