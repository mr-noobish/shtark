use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

    //literal types
    Null,
    Number,
    Identifier,

    //keywords
    Let,
    Const,

    //grouping * operators
    Equals,
    Semicolon,
    OpenParen,
    CloseParen,
    BinaryOperator,
    EOF, // end of file
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tvalue: String,
    pub ttype: TokenType,
}

pub fn token(tvalue: String, ttype: TokenType) -> Token {
    Token { tvalue, ttype }
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn is_skippable(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn make_hashmap() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert(String::from("let"), TokenType::Let);
    keywords.insert(String::from("const"), TokenType::Const);
    keywords.insert(String::from("null"), TokenType::Null);
    return keywords
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src = source_code.chars().peekable();
    let keywords = make_hashmap();

    while let Some(&current) = src.peek() {
        if current == '(' {
            tokens.push(token(current.to_string(), TokenType::OpenParen));
            src.next();
        } else if current == ')' {
            tokens.push(token(current.to_string(), TokenType::CloseParen));
            src.next();
        } else if "+-*/%".contains(current) {
            tokens.push(token(current.to_string(), TokenType::BinaryOperator));
            src.next();
        } else if current == '=' {
            tokens.push(token(current.to_string(), TokenType::Equals));
            src.next();
        } else if current == ';' {
            tokens.push(token(current.to_string(), TokenType::Semicolon));
            src.next();
        } else if is_digit(current) {
            let mut num = String::new();
            while let Some(&next) = src.peek() {
                if is_digit(next) {
                    num.push(next);
                    src.next();
                } else {
                    break;
                }
            }
            tokens.push(token(num, TokenType::Number));
        } else if is_alpha(current) {
            let mut ident = String::new();
            while let Some(&next) = src.peek() {
                if is_alpha(next) {
                    ident.push(next);
                    src.next();
                } else {
                    break;
                }
            }
            if let Some(token_type) = keywords.get(&ident) { // remember typeof reserved == "number"
                tokens.push(token(ident, token_type.clone()));
            } else {
                tokens.push(token(ident, TokenType::Identifier));
            }
        } else if is_skippable(current) {
            src.next(); // Skip whitespace
        } else {
            println!("Unrecognized character found: {}", current);
            std::process::exit(1);
        }
    }
    tokens.push(token(String::from("EndOfFile"), TokenType::EOF));
    tokens
}
