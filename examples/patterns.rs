use std::time::Duration;

use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::SineWave,
};

fn main() {
    let duration = Duration::from_millis(1500);
    let mut pattern_1 = Audact::new(duration);

    let n_1 = std_note_freq(0);
    pattern_1.channel(
        SineWave,
        0.7f32,
        Processing::default(),
        vec![
            n_1, n_1, 0f32, 0f32, n_1, n_1, 0f32, 0f32, n_1, n_1, 0f32, 0f32, n_1, 0f32, 0f32, 0f32,
        ],
    );

    let duration = Duration::from_millis(1500);
    let mut pattern_2 = Audact::new(duration);
    let n_2 = std_note_freq(4);
    pattern_2.channel(SineWave, 0.7f32, Processing::default(), n_2);

    // play the patterns one after another
    pattern_1.run(1);
    pattern_2.run(1);
}
