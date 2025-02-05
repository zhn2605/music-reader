use std::fs;

// let x = 45

enum TokenType {
    Number
}

struct Token {
    value: String,
    type: TokenType,
}