#[derive(PartialEq, Debug)]
pub enum Token {
    Number(i32),
    BinaryOp,
    Identifier(String),
    Let,
    Semicolon,
    Equals,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            },
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
                chars.next();
            },
            'a'..='z' | 'A'..='Z' => {
                let mut id = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() {
                        id.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(id));
                chars.next();
            },
            '+' | '-' => {
                tokens.push(Token::BinaryOp);
                chars.next();

            },
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();

            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}