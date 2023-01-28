use audact::{
    notes::std_note_freq,
    system::{Audact, ProcessingBuilder},
    waves::SineWave,
};
use rand::Rng;

use std::{iter, time::Duration};

fn main() {
    let seq_len = 16;

    let duration = Duration::from_millis(100);
    let mut audact = Audact::new(duration);

    let mut rng = rand::thread_rng();

    let seq: Vec<f32> = (0..(seq_len as f32 / 4f32) as usize)
        .flat_map(|_| iter::repeat(std_note_freq(rng.gen_range(-12..12))).take(4))
        .collect();

    let processing = ProcessingBuilder::default()
        .attack(Duration::from_millis(300u64))
        .reverb((Duration::from_millis(200), 0.8f32))
        .build()
        .unwrap();

    // single test tone
    audact.channel(SineWave, 0.3f32, processing, seq);

    audact.run(4);
}
