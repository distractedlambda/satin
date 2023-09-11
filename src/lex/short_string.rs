use {
    super::{Error, Result, Token},
    crate::string_pool::StringRef,
    lexical::{parse_integer_options, NumberFormatBuilder},
    logos::{Lexer, Logos},
};

#[derive(Debug, Logos)]
#[logos(error = Error, skip br"\\z[ \f\n\r\t\v]+")]
enum SubToken {
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

enum Kind {
    SingleQuote,
    DoubleQuote,
}

const HEX_ESCAPE_FORMAT: u128 = NumberFormatBuilder::new().mantissa_radix(16).build();

pub fn single_quote_callback(lexer: &mut Lexer<Token>) -> Result<StringRef> {
    callback(lexer, Kind::SingleQuote)
}

pub fn double_quote_callback(lexer: &mut Lexer<Token>) -> Result<StringRef> {
    callback(lexer, Kind::DoubleQuote)
}

fn callback(lexer: &mut Lexer<Token>, kind: Kind) -> Result<StringRef> {
    let mut sub_lexer = SubToken::lexer(lexer.remainder());
    lexer.extras.string_buffer.clear();

    let status = loop {
        let token = match sub_lexer.next() {
            None => break Err(Error::UnclosedStringLiteral),
            Some(Err(e)) => break Err(e),
            Some(Ok(token)) => token,
        };

        match token {
            SubToken::LiteralSegment => lexer
                .extras
                .string_buffer
                .extend_from_slice(sub_lexer.slice()),

            SubToken::SingleQuote => match kind {
                Kind::SingleQuote => break Ok(()),
                Kind::DoubleQuote => lexer.extras.string_buffer.push(b'\''),
            },

            SubToken::DoubleQuote => match kind {
                Kind::SingleQuote => lexer.extras.string_buffer.push(b'"'),
                Kind::DoubleQuote => break Ok(()),
            },

            SubToken::BellEscape => lexer.extras.string_buffer.push(0x07),

            SubToken::BackspaceEscape => lexer.extras.string_buffer.push(0x08),

            SubToken::FormFeedEscape => lexer.extras.string_buffer.push(0x0c),

            SubToken::NewlineEscape => lexer.extras.string_buffer.push(b'\n'),

            SubToken::CarriageReturnEscape => lexer.extras.string_buffer.push(b'\r'),

            SubToken::TabEscape => lexer.extras.string_buffer.push(b'\t'),

            SubToken::VerticalTabEscape => lexer.extras.string_buffer.push(0x0b),

            SubToken::BackslashEscape => lexer.extras.string_buffer.push(b'\\'),

            SubToken::DoubleQuoteEscape => lexer.extras.string_buffer.push(b'"'),

            SubToken::SingleQuoteEscape => lexer.extras.string_buffer.push(b'\''),

            SubToken::HexEscape => lexer.extras.string_buffer.push(
                lexical::parse_with_options::<_, _, HEX_ESCAPE_FORMAT>(
                    &sub_lexer.slice()[3..5],
                    &parse_integer_options::STANDARD,
                )
                .unwrap(),
            ),

            SubToken::DecEscape => lexer.extras.string_buffer.push(
                lexical::parse(&sub_lexer.slice()[1..]).map_err(|_| Error::InvalidDecEscape)?,
            ),

            SubToken::UnicodeEscape => {
                let scalar = lexical::parse_with_options::<i32, _, HEX_ESCAPE_FORMAT>(
                    sub_lexer.slice()[3..].split_last().unwrap().1,
                    &parse_integer_options::STANDARD,
                )
                .map_err(|_| Error::InvalidUnicodeEscape)? as u32;

                match scalar {
                    0..=0x7f => lexer.extras.string_buffer.push(scalar as _),

                    0x80..=0x7ff => lexer.extras.string_buffer.extend_from_slice(&[
                        (0xc0 | (scalar >> 6)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x800..=0xffff => lexer.extras.string_buffer.extend_from_slice(&[
                        (0xe0 | (scalar >> 12)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x10000..=0x1fffff => lexer.extras.string_buffer.extend_from_slice(&[
                        (0xf0 | (scalar >> 18)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x200000..=0x3ffffff => lexer.extras.string_buffer.extend_from_slice(&[
                        (0xf8 | (scalar >> 24)) as _,
                        (0x80 | ((scalar >> 18) & 0x3f)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x4000000..=0x7fffffff => lexer.extras.string_buffer.extend_from_slice(&[
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
    status?;
    Ok(lexer.extras.strings.intern(&lexer.extras.string_buffer))
}
