//! Lexical analysis of Lua source code.

use {
    lexical::{parse_float_options, parse_integer_options, NumberFormatBuilder},
    logos::{FilterResult, Lexer, Logos},
    thiserror::Error,
};

#[derive(Default)]
pub struct Extras {
    pub string_contents: Vec<u8>,
}

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum Error {
    #[default]
    #[error("unexpected byte")]
    UnexpectedByte,

    #[error("unclosed string literal")]
    UnclosedStringLiteral,

    #[error("unclosed long comment")]
    UnclosedLongComment,

    #[error("unclosed long literal")]
    UnclosedLongLiteral,

    #[error("invalid decimal escape sequence in string literal")]
    InvalidDecEscape,

    #[error("invalid unicode escape sequence in string literal")]
    InvalidUnicodeEscape,
}

enum ShortStringKind {
    SingleQuote,
    DoubleQuote,
}

const HEX_INT_FORMAT: u128 = NumberFormatBuilder::new().mantissa_radix(16).build();

const DEC_FLOAT_FORMAT: u128 = NumberFormatBuilder::new().no_special(true).build();

const HEX_FLOAT_FORMAT: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .mantissa_radix(16)
    .build();

fn long_comment<T>(lexer: &mut Lexer<Token>) -> FilterResult<T, Error> {
    let open_len = lexer.span().len() - 2;
    let mut sub_lexer = LongCommentToken::lexer(lexer.remainder());
    loop {
        match sub_lexer.next() {
            None => return FilterResult::Error(Error::UnclosedLongComment),

            Some(Err(e)) => return FilterResult::Error(e),

            Some(Ok(LongCommentToken::LiteralSegment)) => (),

            Some(Ok(LongCommentToken::ClosingLongBracket)) => {
                if sub_lexer.span().len() == open_len {
                    lexer.bump(sub_lexer.span().end);
                    return FilterResult::Skip;
                }
            }
        }
    }
}

fn short_string(lexer: &mut Lexer<Token>, kind: ShortStringKind) -> Result<(), Error> {
    lexer.extras.string_contents.clear();

    let mut sub_lexer = ShortStringToken::lexer(lexer.remainder());

    let status = loop {
        let token = match sub_lexer.next() {
            None => break Err(Error::UnclosedStringLiteral),
            Some(Err(e)) => break Err(e),
            Some(Ok(token)) => token,
        };

        match token {
            ShortStringToken::LiteralSegment => lexer
                .extras
                .string_contents
                .extend_from_slice(sub_lexer.slice()),

            ShortStringToken::SingleQuote => match kind {
                ShortStringKind::SingleQuote => break Ok(()),
                ShortStringKind::DoubleQuote => lexer.extras.string_contents.push(b'\''),
            },

            ShortStringToken::DoubleQuote => match kind {
                ShortStringKind::SingleQuote => lexer.extras.string_contents.push(b'"'),
                ShortStringKind::DoubleQuote => break Ok(()),
            },

            ShortStringToken::BellEscape => lexer.extras.string_contents.push(0x07),

            ShortStringToken::BackspaceEscape => lexer.extras.string_contents.push(0x08),

            ShortStringToken::FormFeedEscape => lexer.extras.string_contents.push(0x0c),

            ShortStringToken::NewlineEscape => lexer.extras.string_contents.push(b'\n'),

            ShortStringToken::CarriageReturnEscape => lexer.extras.string_contents.push(b'\r'),

            ShortStringToken::TabEscape => lexer.extras.string_contents.push(b'\t'),

            ShortStringToken::VerticalTabEscape => lexer.extras.string_contents.push(0x0b),

            ShortStringToken::BackslashEscape => lexer.extras.string_contents.push(b'\\'),

            ShortStringToken::DoubleQuoteEscape => lexer.extras.string_contents.push(b'"'),

            ShortStringToken::SingleQuoteEscape => lexer.extras.string_contents.push(b'\''),

            ShortStringToken::HexEscape => lexer.extras.string_contents.push(
                lexical::parse_with_options::<_, _, HEX_INT_FORMAT>(
                    &sub_lexer.slice()[3..5],
                    &parse_integer_options::STANDARD,
                )
                .unwrap(),
            ),

            ShortStringToken::DecEscape => lexer.extras.string_contents.push(
                lexical::parse(&sub_lexer.slice()[1..]).map_err(|_| Error::InvalidDecEscape)?,
            ),

            ShortStringToken::UnicodeEscape => {
                let scalar = lexical::parse_with_options::<i32, _, HEX_INT_FORMAT>(
                    sub_lexer.slice()[3..].split_last().unwrap().1,
                    &parse_integer_options::STANDARD,
                )
                .map_err(|_| Error::InvalidUnicodeEscape)? as u32;

                match scalar {
                    0..=0x7f => lexer.extras.string_contents.push(scalar as _),

                    0x80..=0x7ff => lexer.extras.string_contents.extend_from_slice(&[
                        (0xc0 | (scalar >> 6)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x800..=0xffff => lexer.extras.string_contents.extend_from_slice(&[
                        (0xe0 | (scalar >> 12)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x10000..=0x1fffff => lexer.extras.string_contents.extend_from_slice(&[
                        (0xf0 | (scalar >> 18)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x200000..=0x3ffffff => lexer.extras.string_contents.extend_from_slice(&[
                        (0xf8 | (scalar >> 24)) as _,
                        (0x80 | ((scalar >> 18) & 0x3f)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x4000000..=0x7fffffff => lexer.extras.string_contents.extend_from_slice(&[
                        (0xfc | (scalar >> 30)) as _,
                        (0x80 | ((scalar >> 24) & 0x3f)) as _,
                        (0x80 | ((scalar >> 18) & 0x3f)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    _ => unreachable!(),
                }
            }
        }
    };

    lexer.bump(sub_lexer.span().end);
    status
}

fn long_string(lexer: &mut Lexer<Token>) -> Result<(), Error> {
    lexer.extras.string_contents.clear();

    let open_len = lexer.span().len();

    if matches!(lexer.slice(), &[b'\n', ..]) {
        lexer.bump(1);
    }

    let mut sub_lexer = LongLiteralToken::lexer(lexer.remainder());

    loop {
        match sub_lexer.next().ok_or(Error::UnclosedLongLiteral)?? {
            LongLiteralToken::LiteralSegment => lexer
                .extras
                .string_contents
                .extend_from_slice(sub_lexer.slice()),

            LongLiteralToken::LineEnding => lexer.extras.string_contents.push(b'\n'),

            LongLiteralToken::ClosingLongBracket => {
                if sub_lexer.span().len() == open_len {
                    break;
                } else {
                    lexer
                        .extras
                        .string_contents
                        .extend_from_slice(sub_lexer.slice())
                }
            }
        }
    }

    lexer.bump(sub_lexer.span().end);
    Ok(())
}

fn dec_int(lexer: &mut Lexer<Token>) -> Numeral {
    match lexical::parse(lexer.slice()) {
        Ok(v) => Numeral::Int(v),

        Err(_) => Numeral::Float(
            lexical::parse_with_options::<f64, _, DEC_FLOAT_FORMAT>(
                lexer.slice(),
                &parse_float_options::STANDARD,
            )
            .unwrap()
            .to_bits(),
        ),
    }
}

fn hex_int(lexer: &mut Lexer<Token>) -> Numeral {
    let mut total = 0i64;

    for c in &lexer.slice()[2..] {
        total = total.wrapping_mul(16);
        total = total.wrapping_add(match c {
            b'0'..=b'9' => (c - b'0') as _,
            b'a'..=b'f' => (c - b'a' + 10) as _,
            _ => (c - b'A' + 10) as _,
        })
    }

    Numeral::Int(total)
}

fn dec_float(lexer: &mut Lexer<Token>) -> Numeral {
    Numeral::Float(
        lexical::parse_with_options::<f64, _, DEC_FLOAT_FORMAT>(
            lexer.slice(),
            &parse_float_options::STANDARD,
        )
        .unwrap()
        .to_bits(),
    )
}

fn hex_float(lexer: &mut Lexer<Token>) -> Numeral {
    Numeral::Float(
        lexical::parse_with_options::<f64, _, HEX_FLOAT_FORMAT>(
            &lexer.slice()[2..],
            &parse_float_options::HEX_FLOAT,
        )
        .unwrap()
        .to_bits(),
    )
}

#[derive(Clone, Copy, Debug)]
pub enum Numeral {
    Int(i64),
    Float(u64),
}

#[derive(Clone, Copy, Debug, Logos)]
#[logos(error = Error, extras = Extras, skip br"[ \f\n\r\t\v]", skip br"--[^\r\n]*")]
pub enum Token {
    #[regex(br"--\[=*\[", long_comment)]
    #[regex(b"[_a-zA-Z][_0-9a-zA-Z]*")]
    Name,

    #[token(b"'", |lex| short_string(lex, ShortStringKind::SingleQuote))]
    #[token(b"\"", |lex| short_string(lex, ShortStringKind::DoubleQuote))]
    #[regex(br"\[=*\[", long_string)]
    String,

    #[regex(b"[0-9]+", dec_int)]
    #[regex(b"0[xX][0-9a-fA-F]+", hex_int)]
    #[regex(br"([0-9]+\.[0-9]*|\.[0-9]+)([eE][+\-]?[0-9]+)?", dec_float)]
    #[regex(
        br"0[xX]([0-9a-fA-F]+\.[0-9a-fA-F]*|\.[0-9a-fA-F]+)([pP][+\-]?[0-9]+)?",
        hex_float
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

#[derive(Debug, Logos)]
#[logos(error = Error, skip br"\\z[ \f\n\r\t\v]+")]
enum ShortStringToken {
    #[regex(br#"[^\\'"\r\n]+"#)]
    LiteralSegment,

    #[token(b"'")]
    SingleQuote,

    #[token(b"\"")]
    DoubleQuote,

    #[token(br"\a")]
    BellEscape,

    #[token(br"\b")]
    BackspaceEscape,

    #[token(br"\f")]
    FormFeedEscape,

    #[token(br"\n")]
    #[regex(br"\\(\r|\n|\r\n|\n\r)")]
    NewlineEscape,

    #[token(br"\r")]
    CarriageReturnEscape,

    #[token(br"\t")]
    TabEscape,

    #[token(br"\v")]
    VerticalTabEscape,

    #[token(br"\\")]
    BackslashEscape,

    #[token(br#"\""#)]
    DoubleQuoteEscape,

    #[token(br"\'")]
    SingleQuoteEscape,

    #[regex(br"\\x[0-9a-fA-F]{2}")]
    HexEscape,

    #[regex(br"\\[0-9]{1,3}")]
    DecEscape,

    #[regex(br"\\u\{[0-9a-fA-F]+\}")]
    UnicodeEscape,
}

#[derive(Debug, Logos)]
#[logos(error = Error)]
enum LongLiteralToken {
    #[regex(b".+")]
    LiteralSegment,

    #[token(b"\r")]
    #[token(b"\n")]
    #[token(b"\r\n")]
    #[token(b"\n\r")]
    LineEnding,

    #[regex(br"\]=*\]")]
    ClosingLongBracket,
}

#[derive(Debug, Logos)]
#[logos(error = Error)]
enum LongCommentToken {
    #[regex(b".+")]
    LiteralSegment,

    #[regex(br"\]=*\]")]
    ClosingLongBracket,
}
