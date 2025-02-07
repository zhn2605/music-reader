use rodio::{source, OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::{SineWave, Mix};
use std::time::Duration;

pub struct Chord {
    notes: Vec<String>,
    duration: f32,
}

pub fn calculate_freq(s: &str) -> f32 {
    let A4: f32 = 440.0;
    let mut note_name: &str = "";
    let mut octave: i32 = 0;

    for (i, c) in s.chars().into_iter().enumerate() {
        if c.is_ascii_digit() {
            note_name = &s[..i];
            octave = c
                .to_digit(10)
                .unwrap() as i32;
            break;
        }
    }

    let half_steps = match note_name {
        "A" => 0,
        "A#" | "Bb" => 1,
        "B" => 2,
        "C" => -9,
        "C#" | "Db" => -8,
        "D" => -7,
        "D#" | "Eb" => -6,
        "E" => -5,
        "F" => -4,
        "F#" | "Gb" => -3,
        "G" => -2,
        "G#" | "Ab" => -1,
        _ => panic!("Invalid note"),
    };

    A4 * 2.0f32.powf((octave - 4) as f32 + half_steps as f32 / 12.0)
}

pub fn play_melody(stream_handle: &rodio::OutputStreamHandle, melody: Vec<&str>) {
    let sink = Sink::try_new(stream_handle)
        .map_err(|_e| format!("Could not create sink"))
        .unwrap();

    for note in melody {
        let source = SineWave::new(calculate_freq(note))
            .take_duration(Duration::from_secs_f32(0.3))
            .amplify(0.1);

        sink.append(source);
    }

    sink.sleep_until_end();
}