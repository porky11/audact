use std::time::Duration;

use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::SineWave,
};

fn main() {
    let duration = Duration::from_millis(1500);
    let mut audact = Audact::new().unwrap();

    let c = std_note_freq(0);

    // single test tone
    audact
        .channel(SineWave, 0.7f32, Processing::default(), c, duration)
        .unwrap();

    audact.run(1);
}
