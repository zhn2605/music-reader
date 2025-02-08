use rodio::{source, OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::{SineWave, Mix};
use std::time::Duration;
use crate::parser::{tokenize, AstNode, Note, Parser};

pub struct MusicSheet {
    bpm: f64,
    notes: Vec<Note>
}

impl MusicSheet {
    pub fn new() -> Self {
        MusicSheet {
            bpm: 120.0,
            notes: Vec::new(),
        }
    }

    pub fn read(source: &str) -> Self {
        let mut sheet = MusicSheet::new();
        
        match Self::parse_music_file(source) {
            Ok(ast) => {    
                for node in ast {
                    match node {
                        AstNode::Bpm(value) => {
                            sheet.bpm = value;
                        },
                        AstNode::MusicSheet { notes } => {
                            sheet.notes = notes;
                        },
                        _ => ()
                    }
                }
            },
            Err(e) => println!("Parser error: {}", e)
        }
        
        sheet
    }

    pub fn parse_music_file(source: &str) -> Result<Vec<AstNode>, String> {
        let tokens = tokenize(source);
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    pub fn play_melody(&mut self, stream_handle: &rodio::OutputStreamHandle) {
        let sink = Sink::try_new(stream_handle)
            .map_err(|_e| format!("Could not create sink"))
            .unwrap();
    
        for n in &self.notes {
            let duration = self.calc_note_duration(n.duration);
            let source = SineWave::new(calculate_freq(&n.note))
                .take_duration(Duration::from_secs_f32(duration))
                .amplify(0.1);
    
            sink.append(source);
        }
    
        sink.sleep_until_end();
    }

    fn calc_note_duration(&self, note_value: f64) -> f32 {
        let base_duration = 60.0 / self.bpm as f32;
        base_duration * note_value as f32
    }
}

pub struct Chord {
    notes: Vec<String>,
    duration: f32,
}

fn calculate_freq(s: &str) -> f32 {
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