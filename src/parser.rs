use std::{fs::{File}, io::{BufReader, Read}};

// let x = 45

#[derive(Debug)]
pub enum TokenType {
    Note,
    Chord,
    Pitch(String),
    Play,
    MusicSheet,
    Bpm(f64),

    Number(f64),

    AssignExpr,
    Identifier(String),
    Comma, // separator
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    Invalid,
    EOF
}

pub fn read_file(file_path:  &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut source_code = String::new();
    buf_reader.read_to_string(&mut source_code)?;
    Ok(source_code)
}

// Lexing
pub fn tokenize(source_code: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut chars = source_code.chars().peekable();
    let mut line_number = 1;

    while let Some(ch) = chars.next() {
        match ch {
            // White space
            ' ' | '\r' | '\t' => (),

            '\n' => { line_number+=1; },

            ',' => { tokens.push(TokenType::Comma); },

            '=' => { tokens.push(TokenType::AssignExpr); },

            // Numbers
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);

                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) || ch == '.' {
                        number.push(ch);
                        chars.next();
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

                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '#' {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let keyword = match identifier.as_str() {
                    "BPM" => TokenType::Bpm(120.0),
                    "Play" => TokenType::Play,
                    "MusicSheet" => TokenType::MusicSheet,
                    "Note" => TokenType::Note,
                    other => {
                        let chars: Vec<char> = other.chars().collect();
                        // println!("Chars: {:?}", chars);
                        match chars.as_slice() {
                            [note, acc, octave] if ('A'..='G').contains(note)
                                && ['b', '#'].contains(acc)
                                && ('0'..='8').contains(octave) => TokenType::Pitch(other.to_string()),
                            [note, octave] if ('A'..='G').contains(note)
                                && ('0'..='8').contains(octave) => TokenType::Pitch(other.to_string()),
                            _ => TokenType::Identifier(other.to_string())
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

#[derive(Debug)]
pub struct Note {
    pub note: String,
    pub duration: f64,
}

#[derive(Debug)]
pub enum AstNode {
    MusicSheet {
        notes: Vec<Note>
    },
    Bpm(f64),
    PlayKeyword
}

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

// Parsing
impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }

    fn advance(&mut self) -> Option<&TokenType> {
        let token = self.tokens.get(self.current);
        self.current += 1;
        token
    }

    fn peek(&self) -> Option<&TokenType> {
        self.tokens.get(self.current)
    }

    pub fn parse_program(&mut self) -> Result<Vec<AstNode>, String> {
        let mut nodes = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                TokenType::Bpm(_) => {
                    nodes.push(self.parse_bpm()?);
                },
                TokenType::MusicSheet => {
                    nodes.push(self.parse_music_sheet()?);
                },
                TokenType:: Play => {
                    self.advance();
                    nodes.push(AstNode::PlayKeyword);
                },
                TokenType::EOF => break,
                _ => return Err(format!("Unexpected token {:?}: ", token))
            }
        }
        Ok(nodes)
    }

    pub fn parse_bpm(&mut self) -> Result<AstNode, String> {
        match self.advance() {
            Some(TokenType::Bpm(_)) => {
                // Look for bpm value
                match self.advance() {
                    Some(TokenType::Number(value)) => Ok(AstNode::Bpm(*value)),
                    _ => Err(String::from("Expected numeric value after BPM"))
                }
            },
            _ => Err(String::from("Expected BPM keyword"))
        }
    }

    fn parse_music_sheet(&mut self) -> Result<AstNode, String> {
        // music sheets should be in this format
        /* 
        MusicSheet = { 
            Note(<note>, <duration>), Note... 
        }
        */

        match self.advance() {
            Some(TokenType::MusicSheet) => (),
            _ => return Err(String::from("Expected Music Sheet"))
        }

        match self.advance() {
            Some(TokenType::AssignExpr) => (),
            _ => return Err(String::from("Expected ="))
        }

        match self.advance() {
            Some(TokenType::OpenBrace) => (),
            _ => return Err(String::from("Expected {"))
        }

        let mut notes = Vec::new();

        while let Some(token) = self.advance() {
            match token {
                TokenType::CloseBrace => {
                    self.advance();
                    break;
                },
                TokenType::Note => {
                    notes.push(self.parse_note()?);
                    
                    if let Some(next_token) = self.peek() {
                        match next_token {
                            // check if valid use of , to separate notes. if at the end of music sheet
                            TokenType::CloseBrace => continue,
                            TokenType::Comma => {
                                self.advance();
                            },
                            _ => return Err(String::from("Expected comma between notes"))
                        }
                    }
                },
                _ => return Err(format!("Unexpected token in music sheet: {:?}", token))
            }
        }

        Ok(AstNode::MusicSheet { notes })
    }

    fn parse_note(&mut self) -> Result<Note, String> {
        match self.advance() {
            Some(TokenType::OpenParen) => (),
            _ => return Err(String::from("Expected ("))
        }

        let note_token = self.advance().ok_or("Expected pitch")?;
        let note = match note_token {
            TokenType::Pitch(value) => value.clone(),
            _ => return Err(String::from("Expected pitch value"))
        };

        println!("Successfully got note: {}", note);

        // comma separating params
        match self.advance() {
            Some(TokenType::Comma) => (),
            _ => return Err(String::from("Expected ,"))
        }

        let duration = if let Some(&TokenType::Number(value)) = self.tokens.get(self.current) {
            self.advance(); 
            value
        } else {
            1.0
        };

        println!("Successfully got duration: {}", duration);

        match self.advance() {
            Some(TokenType::CloseParen) => (),
            _ => return Err(String::from("Expected )"))
        }   

        Ok(Note { note, duration } )
    }
}