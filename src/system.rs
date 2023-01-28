use crate::waves::Wave;

use std::time::Duration;

use derive_builder::Builder;
use rodio::{buffer::SamplesBuffer, source, OutputStream, OutputStreamHandle, Sink, Source};

/// Struct for the main audact system
pub struct Audact {
    /// The output stream that will actually be played through
    /// kept here as won't work if it is dropped
    _output_stream: OutputStream,
    /// The output stream handle that audact will play through
    output_stream_handle: OutputStreamHandle,
    /// Vec of voice channels that audact will play
    channels: Vec<Channel>,
    /// Sample rate
    sample_rate: u32,
    /// Samples needed per step
    samples_needed: f32,
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
    pub fn new(bpm_duration: Duration) -> Audact {
        let (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sample_rate = 44100f32;
        let samples_needed = sample_rate * bpm_duration.as_secs_f32();

        Audact {
            _output_stream,
            output_stream_handle,
            channels: Vec::new(),
            sample_rate: sample_rate as u32,
            samples_needed,
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
        seq: Vec<f32>,
    ) {
        // create the sink to play from
        let sink = Sink::try_new(&self.output_stream_handle).unwrap();
        sink.pause();

        let sample_rate = self.sample_rate as f32;
        let steps = seq.len() as f32;
        let samples_needed = self.samples_needed;
        let total_samples_needed = samples_needed * steps as f32;

        // Create the basic waveform samples
        let mut source: Vec<f32> = (0u64..total_samples_needed as u64)
            .map(move |t| {
                let t = t as f32;
                // Silence if not playing in this step
                let s_t = total_samples_needed / t;
                let freq = seq[(steps / s_t).floor() as usize];
                if freq == 0f32 {
                    return 0f32;
                }

                let n_t = t / (samples_needed - 1.0);

                // Calc the freq for the wave
                let freq = t * freq / sample_rate;
                //let freq = freq / 2.0f32.powf((n_t * std::f32::consts::PI / 2.0).sin());
                // Call the wave gen fn
                (wave.calculate(freq) * 2.0 - 1.0) * volume.calculate(n_t)
            })
            .collect();

        Audact::smooth_source(&mut source);

        // Create the processing chain and channel
        let channel = Channel {
            sink,
            source,
            processing,
        };

        self.channels.push(channel);
    }

    /// Start playing the audio `bars` times
    pub fn start(&self, bars: i32) {
        // grab some values from the stuct to be moved
        let sample_rate = self.sample_rate;
        // The repeats of the sequence
        for _ in 0..bars {
            for chan in &self.channels {
                // create buffer
                let samples = chan.source.clone();
                let sample_buffer = vec![SamplesBuffer::new(2, sample_rate, samples)];
                // create the source
                let source = source::from_iter(sample_buffer)
                    .buffered()
                    .fade_in(chan.processing.attack)
                    .low_pass(chan.processing.filter.1 as u32)
                    .reverb(chan.processing.reverb.0, chan.processing.reverb.1)
                    .amplify(chan.processing.gain);
                // add source to sink queue
                chan.sink.append(source);
            }
        }

        // Play all the channels
        for chan in &self.channels {
            chan.sink.play();
        }
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
