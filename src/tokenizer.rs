#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}


pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '(' => { chars.next(); tokens.push(Token::LeftParen); }
            ')' => { chars.next(); tokens.push(Token::RightParen); }
            '/' => { chars.next(); tokens.push(Token::Slash); }
            c if c.is_ascii_digit() => {
                let mut number = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        number.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match number.parse() {
                    Ok(number) => tokens.push(Token::Number(number)),
                    Err(_) => {
                        return Err(String::from("Number could not be parsed."))
                    }
                }
            },
            c if c.is_whitespace() => { chars.next(); }
            _ =>  {
                return Err(format!("Invalid token: '{}'", c));
            }
        }
    }
    Ok(tokens)
}
