use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Whitespace,
    Location(i32),
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \n]+"# => Token::Whitespace,
    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Location(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }
}

pub struct Lexer<'a> {
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            remaining: s,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some((tok, new_remaining)) = next_token(self.remaining) {
            self.remaining = new_remaining;
            Some(tok)    
        } else {
            None
        }
    }
}


