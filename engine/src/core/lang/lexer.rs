
use super::token::{Token, TokenKind};
struct Lexer<'source> {
    src:    &'source str,
    inx:    usize,
    tokens: Vec<Token<'source>>,
}


trait Alphanumeric {
    fn is_alphabetic(&self) -> bool;
    fn is_alphanumeric(&self) -> bool;
}

impl Alphanumeric for &str  {
    fn is_alphabetic(&self) -> bool {
        // unsafe because this is never possible and rust is stupid 
        unsafe { self.chars().next().unwrap_unchecked() }.is_alphabetic()
    }

    fn is_alphanumeric(&self) -> bool {
        unsafe { self.chars().next().unwrap_unchecked() }.is_alphanumeric()
    }
}

impl<'source> Lexer<'source> {
    fn get(&self, n: usize) -> Option<&'source str> {
        self.src.get(n..=n)
    }

    fn peek(&self) -> Option<&'source str> {
        self.get(self.inx+1)
    }

    fn simple(&mut self, kind: TokenKind, len: usize) {
        self.inx += len;
        self.tokens.push(Token { kind, 
            lit: unsafe { self.src.get_unchecked(self.inx.unchecked_sub(len)..=self.inx)} })
    }

    fn consume_while<F: FnMut(&'source str) -> bool>(&mut self, start: usize, mut f: F) -> Option<&'source str> {
        loop {
            let Some(c) = self.get(self.inx) else {
                panic!("Unexpected EOF"); 
                return None;
            };

            if f(c) { self.inx += 1; }
            else { return Some(unsafe { self.src.get_unchecked(start..=self.inx) }); }
        }
    }

    pub fn lex(&self,src: &'source str) -> Vec<Token> {
        let mut lex = Lexer { inx: 0, tokens: Vec::with_capacity(src.len()), src };

        while let Some(lit) = lex.get(lex.inx) {
            let start_pos = lex.inx;

            match lit {
                l if l.is_alphabetic() || l == "_" => {
                    let kind = match self.consume_while(start_pos, |s| s.is_alphabetic() || s.is_alphanumeric() || s == "_") {
                        Some("const") => TokenKind::Const,
                        None        => panic!("nuh uh"),
                    };
                    
                    self.tokens.push(Token { kind, lit  })
                },


                "$"  => lex.simple(TokenKind::Dollar,1),
                "="  => match lex.peek() {
                    Some("=") => lex.simple(TokenKind::EqualEqual, 2),
                    _         => lex.simple(TokenKind::Equal, 1),
                },
                _ => todo!()
            }
        }

        lex.tokens
    }
}

