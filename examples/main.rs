use audact::{
    notes::std_note_freq,
    system::{Audact, Processing, ProcessingBuilder},
    waves::{NoiseWave, SawWave, SineWave, SquareWave},
};

use std::time::Duration;

fn main() {
    let duration = Duration::from_millis(1500);
    let mut audact = Audact::new().unwrap();

    let lead_processing = ProcessingBuilder::default()
        .attack(Duration::from_millis(100u64))
        .reverb((Duration::from_millis(200), 0.8f32))
        .build()
        .unwrap();

    let default_processing = Processing::default();

    //lead
    let l_1 = std_note_freq(0.0);
    let l_2 = std_note_freq(-2.0);
    let l_3 = std_note_freq(-4.0);
    audact
        .channel(
            SawWave,
            0.1f32,
            lead_processing,
            vec![
                l_1, l_2, l_3, l_3, l_1, l_2, l_3, l_3, l_1, l_2, l_3, l_3, l_1, l_2, l_3, l_3,
            ],
            duration,
        )
        .unwrap();

    //pad
    let p_1 = std_note_freq(-12.0);
    let p_2 = std_note_freq(-14.0);
    let p_3 = std_note_freq(-16.0);
    audact
        .channel(
            SquareWave,
            0.1f32,
            default_processing,
            vec![p_1, p_1, p_2, p_3],
            duration,
        )
        .unwrap();

    let b_1 = std_note_freq(-24.0);
    let b_2 = std_note_freq(-26.0);
    //bass
    audact
        .channel(
            SineWave,
            0.1f32,
            default_processing,
            vec![b_1, b_2],
            duration,
        )
        .unwrap();

    // percussion
    audact
        .channel(
            NoiseWave,
            0.2f32,
            default_processing,
            vec![
                b_1, 0f32, l_1, 0f32, b_1, 0f32, l_1, 0f32, b_1, 0f32, l_1, 0f32, b_1, 0f32, l_1,
                0f32,
            ],
            duration,
        )
        .unwrap();

    audact.run(1);
}
