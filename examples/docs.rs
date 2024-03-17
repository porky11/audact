use std::time::Duration;

use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::{SineWave, SquareWave},
};

fn main() {
    let duration = Duration::from_millis(500);
    let mut audact = Audact::new().unwrap();

    let default_processing = Processing::default();
    let n_1 = std_note_freq(0.0);
    let n_2 = std_note_freq(2.0);

    audact
        .channel(
            SineWave,
            1f32,
            default_processing,
            vec![n_1, 0f32, 0f32, 0f32],
            duration,
        )
        .unwrap();
    audact
        .channel(
            SquareWave,
            1f32,
            default_processing,
            vec![0f32, 0f32, n_2, 0f32],
            duration,
        )
        .unwrap();

    audact.run(4);
}
