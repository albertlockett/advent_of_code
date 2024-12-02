pub struct Lexer<'a, T> {
    remaining: &'a str,
    next_token: Box<dyn Fn(&'a str) -> Option<(T, &'a str)>>,
}

impl<'a, T> Lexer<'a, T> {
    pub fn new(
        s: &'a str,
        next_token: Box<dyn Fn(&'a str) -> Option<(T, &'a str)>>,
    ) -> Lexer<'a, T> {
        Lexer::<T> {
            remaining: s,
            next_token,
        }
    }
}

impl<T> Iterator for Lexer<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Some((tok, new_remaining)) = (self.next_token)(self.remaining) {
            self.remaining = new_remaining;
            Some(tok)
        } else {
            None
        }
    }
}

pub mod numberic {
    use plex::lexer;

    pub enum Token {
        Whitespace,
        EndLine,
        Number(i32),
    }

    lexer! {
        pub fn next_token(text: 'a) -> Token;
        r#"[\n]"# => Token::EndLine,
        r#"[ ]+"# => Token::Whitespace,
        r#"[0-9]+"# => {
            if let Ok(i) = text.parse() {
                Token::Number(i)
            } else {
                panic!("integer {} is out of range", text)
            }
        }

    }
}
