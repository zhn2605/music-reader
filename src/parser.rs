use std::fs;

// let x = 45

#[derive(Debug)]
enum TokenType {
    Note(String),
    Bpm(f64),

    Identifier(String),
    Number(f64),

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    Invalid,
    EOF
}

struct Lexer {
    value: String,
    token: TokenType,
} 

pub fn tokenize(source_code: &String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut chars = source_code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // White space
            ' ' => { chars.next(); },

            // Numbers
            '0'..='9' => {
                let mut number = String::new();
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
            
            // Note
            'A'..='G' => {
                let mut note = String::new();
                if  
            }

            _ => { println!("Unexpected char: {}", &ch); }
        }
    }


    tokens
}