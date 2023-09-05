//! Lexical analysis of Lua source code.

pub use {error::Error, numeral::Numeral};
use {
    logos::{Lexer, Logos},
    std::ops::Range,
};

mod error;
mod long_comment;
mod long_literal;
mod numeral;
mod short_string;

pub type Result<T> = std::result::Result<T, Error>;

pub struct TokenStream<'source> {
    lexer: Lexer<'source, Token>,
    backtrack_stack: Vec<SpannedToken>,
}

pub type SpannedToken = (Token, Range<usize>);

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
            backtrack_stack: Vec::new(),
        }
    }

    pub fn source(&self) -> &'source [u8] {
        self.lexer.source()
    }

    pub fn backtrack(&mut self, token: SpannedToken) {
        self.backtrack_stack.push(token)
    }
}

impl<'source> Iterator for TokenStream<'source> {
    type Item = Result<SpannedToken>;

    fn next(&mut self) -> Option<Result<SpannedToken>> {
        Some(if let Some(token) = self.backtrack_stack.pop() {
            Ok(token)
        } else {
            let token = self.lexer.next()?;
            token.map(|token| (token, self.lexer.span()))
        })
    }
}
