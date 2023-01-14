use audact::{
    notes::std_note_freq,
    system::{Audact, Processing},
    waves::SineWave,
};

fn main() {
    let mut audact = Audact::new(100, 4f32);

    let c = std_note_freq(0);

    // single test tone
    audact.channel(
        SineWave,
        0.7f32,
        Processing::default(),
        vec![c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c],
    );

    audact.start(1);
}
