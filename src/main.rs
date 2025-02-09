use std::{fs::{self}, io::{self, Write}, str::Bytes};
use parser::tokenize;
use rodio::{source, OutputStream, OutputStreamHandle, Sink, Source};

mod parser;
use crate::parser::Parser;
mod music;
use crate::music::MusicSheet;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|_e| format!("Could not create outputstream"))
        .unwrap();
    let mut file_name = String::new();
    
    // Loop until input is recieved
    /*
    loop {
        print!("Enter file path: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut file_name) {
            Ok(_) => {
                let file_name = file_name.trim(); // crucial to fixing file-read error due to '\n' char
                match fs::read_to_string(file_name) {
                    Ok(_) => break,
                    Err(e) => println!("Can not access file (check file director): {}", e)
                }
            },
            Err(_) => println!("Failed to read input. Try again."),
        }
    }
    */

    // let source_code= parser::read_file(&file_name.trim()).unwrap();
    let source_code = parser::read_file("music_sheets/test.txt").unwrap();
    let tokens = tokenize(&source_code);
    println!("{:?}", tokens);
    
    let mut music_sheet = MusicSheet::read(&source_code);
    music_sheet.play_melody(&stream_handle);
}