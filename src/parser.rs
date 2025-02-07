use std::{fs::{self, File}, io::Read, path::PathBuf};

// let x = 45

#[derive(Debug)]
pub enum TokenType {
    Note(String),
    Play,
    MusicSheet,
    Bpm(f64),

    Number(f64),

    AssignExpr,
    Identifier,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    Invalid,
    EOF
}

struct Lexer {
    value: String,
    token: TokenType,
}

pub fn read_file(file_path:  &str) -> String {
    println!("Reading file");

    let source_code = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| panic!("Could not read file: {}", e));

    println!("Successfully read file");
    source_code
}

pub fn tokenize(source_code: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut chars = source_code.chars().peekable();
    let mut line_number = 1;

    println!("{}", &source_code);

    while let Some(ch) = chars.next() {
        match ch {
            // White space
            ' ' | '\r' | '\t' => (),

            '\n' => { line_number+=1; }

            // Numbers
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);

                while let Some(ch) = chars.next() {
                    if ch.is_digit(10) || ch == '.' {
                        number.push(ch);
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::Number(number.parse().unwrap()));
            },
            
            // Identifiers
            'A'..='Z' | 'a'..='z' => {
                let mut identifier = String::new();
                identifier.push(ch);

                while let Some(ch) = chars.next() {
                    if ch.is_ascii_alphanumeric() {
                        identifier.push(ch);
                    } else {
                        break;
                    }
                }

                let keyword = match identifier.as_str() {
                    "BPM" => TokenType::Bpm(120.0),
                    "let" => TokenType::AssignExpr,
                    "Play" => TokenType::Play,
                    "MusicSheet" => TokenType::MusicSheet,
                    other => {
                        let chars: Vec<char> = other.chars().collect();
                        match chars.as_slice() {
                            [note, octave] if ('A'..='G').contains(note) 
                                && ('0'..='8').contains(octave) => TokenType::Note(other.to_string()),
                            [note, acc, octave] if ('A'..='G').contains(note)
                                && ['b', '#'].contains(acc)
                                && ('0'..='8').contains(octave) => TokenType::Note(other.to_string()),
                            _ => TokenType::Identifier
                        }
                    }
                };
                tokens.push(keyword);
            },

            '(' => {
                tokens.push(TokenType::OpenParen);
            },
            ')' => {
                tokens.push(TokenType::CloseParen);
            },
            '{' => {
                tokens.push(TokenType::OpenBrace);
            },
            '}' => {
                tokens.push(TokenType::CloseBrace);
            },
            ';' => {
                tokens.push(TokenType::Semicolon);
            },

            _ => {
                tokens.push(TokenType::Invalid);
                panic!("Unexpected char: {}\nLine: {}", &ch, line_number);
            }
        }
    }

    tokens.push(TokenType::EOF);
    tokens
}