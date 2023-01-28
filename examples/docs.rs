use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::{SineWave, SquareWave},
};

fn main() {
    let mut audact = Audact::new(120, 4f32);

    let default_processing = Processing::default();
    let n_1 = std_note_freq(0);
    let n_2 = std_note_freq(2);

    audact.channel(
        SineWave,
        1f32,
        default_processing,
        vec![
            n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32, 0f32, n_1, 0f32, 0f32,
            0f32,
        ],
    );
    audact.channel(
        SquareWave,
        1f32,
        default_processing,
        vec![
            0f32, 0f32, n_2, 0f32, 0f32, 0f32, n_2, 0f32, 0f32, 0f32, n_2, 0f32, 0f32, 0f32, n_2,
            0f32,
        ],
    );

    audact.run(1);
}
