//! Lexical analysis of Lua source code.

use {
    crate::string_pool::{StringPool, StringRef},
    logos::Logos,
    std::rc::Rc,
};
pub use {error::Error, numeral::Numeral};

mod error;
mod long_comment;
mod long_literal;
mod numeral;
mod short_string;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Extras {
    strings: Rc<StringPool>,
    string_buffer: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Logos)]
#[logos(error = Error, extras = Extras, skip br"[ \f\n\r\t\v]", skip br"--[^\r\n]*")]
pub enum Token {
    #[regex(br"--\[=*\[", long_comment::callback)]
    #[regex(b"[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.extras.strings.intern(lex.slice()))]
    Name(StringRef),

    #[token(b"'", short_string::single_quote_callback)]
    #[token(b"\"", short_string::double_quote_callback)]
    #[regex(br"\[=*\[", long_literal::callback)]
    String(StringRef),

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
