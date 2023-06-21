use crate::token::{Kind, Token, TokenValue};
use dispatch::{Dispatch, DISPATCHER};

mod dispatch;

pub struct Lexer<'a> {
    bytes: &'a [u8],
    curr: usize,
    pub file_id: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        Self {
            bytes: source.as_bytes(),
            curr: 0,
            file_id,
        }
    }

    fn lex_token(&mut self) -> Token {
        let start = self.curr;
        let kind = self.get_kind();
        let end = self.curr;

        let value = match kind {
            _ => TokenValue::None,
        };

        return Token {
            start,
            end,
            kind,
            value,
        };
    }

    fn get_kind(&mut self) -> Kind {
        use Dispatch::*;
        let byte = unsafe { *self.bytes.get_unchecked(self.curr) };
        let dispatched = Self::lookup(byte);

        return match dispatched {
            PLS => self.resolve_plus(),
            QOT => self.resolve_str_literal(),
            IDT => self.resolve_identifier(),
            _ => Kind::Eof,
        };
    }

    // Stolen from RSLint
    fn lookup(byte: u8) -> Dispatch {
        // Safety: our lookup table maps all values of u8, so its impossible for a u8 to be out of bounds
        unsafe { *DISPATCHER.get_unchecked(byte as usize) }
    }

    fn resolve_plus(&self) -> Kind {
        return Kind::Plus;
    }

    fn resolve_str_literal(&self) -> Kind {
        todo!("");
    }

    fn resolve_identifier(&self) -> Kind {
        return Kind::Identifier;
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let _ = self.lex_token();
        todo!("Implementing this later, just getting the LSP to shut up for now")
    }
}
