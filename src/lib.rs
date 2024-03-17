/*!
Minimalist synth and sequencing lib.

Contains:
- Simple sine, square, saw and noise waveforms.
- Hard-edge cut-off filters.
- Basic sequencing of a single pattern.

Usage:

```no_run
use std::time::Duration;

use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::{SineWave, SquareWave},
};

let duration = Duration::from_millis(2000);
let mut audact = Audact::new().unwrap();

let default_processing = Processing::default();
let n_1 = std_note_freq(0.0);
let n_2 = std_note_freq(2.0);

audact
    .channel(
        SineWave,
        1f32,
        default_processing,
        vec![
            n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32,
            0f32, 0f32,
        ],
        duration,
    )
    .unwrap();
audact
    .channel(
        SquareWave,
        1f32,
        default_processing,
        vec![
            0f32, 0f32, n_2, 0f32, 0f32, 0f32, n_2, 0f32, 0f32, 0f32, n_2, 0f32, 0f32, 0f32,
            n_2, 0f32,
        ],
        duration,
    )
    .unwrap();

audact.run(1);
```

*/

#![deny(missing_docs)]

/// Module containing note frequencies
pub mod notes;
/// Module containing the pitchers
pub mod pitchers;
/// Module for the main audact system
pub mod system;
/// Module containing some common wave types
pub mod waves;
