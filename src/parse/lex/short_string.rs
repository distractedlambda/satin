use {
    super::{Error, Result},
    lexical::{parse_integer_options, NumberFormatBuilder},
    logos::{Lexer, Logos},
    std::borrow::Cow,
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

pub fn single_quote_callback<'source, T>(lexer: &mut Lexer<'source, T>) -> Result<Cow<'source, [u8]>>
where
    T: Logos<'source, Source = [u8]>,
{
    callback(lexer, Kind::SingleQuote)
}

pub fn double_quote_callback<'source, T>(lexer: &mut Lexer<'source, T>) -> Result<Cow<'source, [u8]>>
where
    T: Logos<'source, Source = [u8]>,
{
    callback(lexer, Kind::DoubleQuote)
}

fn callback<'source, T>(lexer: &mut Lexer<'source, T>, kind: Kind) -> Result<Cow<'source, [u8]>>
where
    T: Logos<'source, Source = [u8]>,
{
    let mut sub_lexer = SubToken::lexer(lexer.remainder());
    let mut contents = Vec::new();

    let status = loop {
        let token = match sub_lexer.next() {
            None => break Err(Error::UnclosedStringLiteral),
            Some(Err(e)) => break Err(e),
            Some(Ok(token)) => token,
        };

        match token {
            SubToken::LiteralSegment => contents.extend_from_slice(sub_lexer.slice()),

            SubToken::SingleQuote => match kind {
                Kind::SingleQuote => break Ok(()),
                Kind::DoubleQuote => contents.push(b'\''),
            },

            SubToken::DoubleQuote => match kind {
                Kind::SingleQuote => contents.push(b'"'),
                Kind::DoubleQuote => break Ok(()),
            },

            SubToken::BellEscape => contents.push(0x07),

            SubToken::BackspaceEscape => contents.push(0x08),

            SubToken::FormFeedEscape => contents.push(0x0c),

            SubToken::NewlineEscape => contents.push(b'\n'),

            SubToken::CarriageReturnEscape => contents.push(b'\r'),

            SubToken::TabEscape => contents.push(b'\t'),

            SubToken::VerticalTabEscape => contents.push(0x0b),

            SubToken::BackslashEscape => contents.push(b'\\'),

            SubToken::DoubleQuoteEscape => contents.push(b'"'),

            SubToken::SingleQuoteEscape => contents.push(b'\''),

            SubToken::HexEscape => contents.push(
                lexical::parse_with_options::<_, _, HEX_ESCAPE_FORMAT>(
                    &sub_lexer.slice()[3..5],
                    &parse_integer_options::STANDARD,
                )
                .unwrap(),
            ),

            SubToken::DecEscape => contents.push(
                lexical::parse(&sub_lexer.slice()[1..]).map_err(|_| Error::InvalidDecEscape)?,
            ),

            SubToken::UnicodeEscape => {
                let scalar = lexical::parse_with_options::<i32, _, HEX_ESCAPE_FORMAT>(
                    sub_lexer.slice()[3..].split_last().unwrap().1,
                    &parse_integer_options::STANDARD,
                )
                .map_err(|_| Error::InvalidUnicodeEscape)? as u32;

                match scalar {
                    0..=0x7f => contents.push(scalar as _),

                    0x80..=0x7ff => contents.extend_from_slice(&[
                        (0xc0 | (scalar >> 6)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x800..=0xffff => contents.extend_from_slice(&[
                        (0xe0 | (scalar >> 12)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x10000..=0x1fffff => contents.extend_from_slice(&[
                        (0xf0 | (scalar >> 18)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x200000..=0x3ffffff => contents.extend_from_slice(&[
                        (0xf8 | (scalar >> 24)) as _,
                        (0x80 | ((scalar >> 18) & 0x3f)) as _,
                        (0x80 | ((scalar >> 12) & 0x3f)) as _,
                        (0x80 | ((scalar >> 6) & 0x3f)) as _,
                        (0x80 | (scalar & 0x3f)) as _,
                    ]),

                    0x4000000..=0x7fffffff => contents.extend_from_slice(&[
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
    Ok(Cow::Owned(contents))
}
