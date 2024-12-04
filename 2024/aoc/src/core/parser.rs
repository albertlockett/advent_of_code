#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

pub struct Lexer<'a, T> {
    original: &'a str,
    remaining: &'a str,
    next_token: Box<dyn Fn(&'a str) -> Option<(T, &'a str)>>,
}

impl<'a, T> Lexer<'a, T> {
    pub fn new(
        s: &'a str,
        next_token: Box<dyn Fn(&'a str) -> Option<(T, &'a str)>>,
    ) -> Lexer<'a, T> {
        Lexer::<T> {
            original: s,
            remaining: s,
            next_token,
        }
    }
}

impl<T> Iterator for Lexer<'_, T> {
    type Item = (T, Span);
    fn next(&mut self) -> Option<(T, Span)> {
        if let Some((tok, new_remaining)) = (self.next_token)(self.remaining) {
            let lo = self.original.len() - self.remaining.len();
            let hi = self.original.len() - new_remaining.len();
            self.remaining = new_remaining;
            Some((tok, Span { lo, hi }))
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
        Other,
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
        "." => Token::Other,

    }
}
