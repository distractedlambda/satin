//! Lexical analysis of Lua source code.

pub use {error::Error, numeral::Numeral};
use {
    logos::{Lexer, Logos},
    std::{collections::VecDeque, ops::Range},
};

mod error;
mod long_comment;
mod long_literal;
mod numeral;
mod short_string;

pub type Result<T> = std::result::Result<T, Error>;

pub struct TokenStream<'source> {
    lexer: Lexer<'source, Token>,
    lookahead_buffer: VecDeque<SpannedToken>,
}

pub struct SpannedToken {
    pub token: Token,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Logos)]
#[logos(error = Error, skip br"[ \f\n\r\t\v]", skip br"--[^\r\n]*")]
pub enum Token {
    #[regex(br"--\[=*\[", long_comment::callback)]
    #[regex(b"[_a-zA-Z][_0-9a-zA-Z]*")]
    Name,

    #[token(b"'", short_string::single_quote_callback)]
    #[token(b"\"", short_string::double_quote_callback)]
    #[regex(br"\[=*\[", long_literal::callback)]
    String(Vec<u8>),

    #[regex(b"[0-9]+", numeral::dec_int_callback)]
    #[regex(b"0[xX][0-9a-fA-F]+", numeral::hex_int_callback)]
    #[regex(
        br"([0-9]+\.[0-9]*|\.[0-9]+)([eE][+\-]?[0-9]+)?",
        numeral::dec_float_callback
    )]
    #[regex(
        br"0[xX]([0-9a-fA-F]+\.[0-9a-fA-F]*|\.[0-9a-fA-F]+)([pP][+\-]?[0-9]+)?",
        numeral::hex_float_callback
    )]
    Numeral(Numeral),

    #[token(b"and")]
    KwAnd,

    #[token(b"break")]
    KwBreak,

    #[token(b"do")]
    KwDo,

    #[token(b"else")]
    KwElse,

    #[token(b"elseif")]
    KwElseif,

    #[token(b"end")]
    KwEnd,

    #[token(b"false")]
    KwFalse,

    #[token(b"for")]
    KwFor,

    #[token(b"function")]
    KwFunction,

    #[token(b"goto")]
    KwGoto,

    #[token(b"if")]
    KwIf,

    #[token(b"in")]
    KwIn,

    #[token(b"local")]
    KwLocal,

    #[token(b"nil")]
    KwNil,

    #[token(b"not")]
    KwNot,

    #[token(b"or")]
    KwOr,

    #[token(b"repeat")]
    KwRepeat,

    #[token(b"return")]
    KwReturn,

    #[token(b"then")]
    KwThen,

    #[token(b"true")]
    KwTrue,

    #[token(b"until")]
    KwUntil,

    #[token(b"while")]
    KwWhile,

    #[token(b"+")]
    Plus,

    #[token(b"-")]
    Minus,

    #[token(b"*")]
    Star,

    #[token(b"/")]
    Slash,

    #[token(b"%")]
    Percent,

    #[token(b"^")]
    Caret,

    #[token(b"#")]
    Hash,

    #[token(b"&")]
    Ampersand,

    #[token(b"~")]
    Tilde,

    #[token(b"|")]
    Pipe,

    #[token(b"<<")]
    LAngle2,

    #[token(b">>")]
    RAngle2,

    #[token(b"//")]
    Slash2,

    #[token(b"==")]
    Equals2,

    #[token(b"~=")]
    TildeEquals,

    #[token(b"<=")]
    LAngleEquals,

    #[token(b">=")]
    RAngleEquals,

    #[token(b"<")]
    LAngle,

    #[token(b">")]
    RAngle,

    #[token(b"=")]
    Equals,

    #[token(b"(")]
    LParen,

    #[token(b")")]
    RParen,

    #[token(b"{")]
    LCurly,

    #[token(b"}")]
    RCurly,

    #[token(b"[")]
    LSquare,

    #[token(b"]")]
    RSquare,

    #[token(b"::")]
    Colon2,

    #[token(b";")]
    Semicolon,

    #[token(b":")]
    Colon,

    #[token(b",")]
    Comma,

    #[token(b".")]
    Dot,

    #[token(b"..")]
    Dot2,

    #[token(b"...")]
    Dot3,
}

impl<'source> TokenStream<'source> {
    pub fn new(source: &'source [u8]) -> Self {
        Self {
            lexer: Token::lexer(source),
            lookahead_buffer: VecDeque::new(),
        }
    }

    pub fn source(&self) -> &'source [u8] {
        self.lexer.source()
    }

    fn grab_token(&mut self) -> Result<Option<SpannedToken>> {
        if let Some(token) = self.lexer.next() {
            let token = token?;
            Ok(Some(SpannedToken {
                token,
                span: self.lexer.span(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn lookahead(&mut self, offset: usize) -> Result<Option<&SpannedToken>> {
        while self.lookahead_buffer.len() <= offset {
            if let Some(token) = self.grab_token()? {
                self.lookahead_buffer.push_back(token)
            } else {
                return Ok(None);
            }
        }

        Ok(Some(&self.lookahead_buffer[offset]))
    }

    pub fn advance(&mut self, count: usize) {
        for _ in 0..count {
            self.lookahead_buffer.pop_front().unwrap();
        }
    }
}
