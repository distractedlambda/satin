use std::borrow::Cow;

use {
    super::{Error, Result},
    logos::{Lexer, Logos},
};

#[derive(Debug, Logos)]
#[logos(error = Error)]
enum SubToken {
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

pub fn callback<'source, T>(lexer: &mut Lexer<'source, T>) -> Result<Cow<'source, [u8]>>
where
    T: Logos<'source, Source = [u8]>,
{
    let open_len = lexer.span().len();

    if matches!(lexer.slice(), &[b'\n', ..]) {
        lexer.bump(1);
    }

    let mut sub_lexer = SubToken::lexer(lexer.remainder());
    let mut contents = Vec::new();

    loop {
        match sub_lexer.next().ok_or(Error::UnclosedLongLiteral)?? {
            SubToken::LiteralSegment => contents.extend_from_slice(sub_lexer.slice()),
            SubToken::LineEnding => contents.push(b'\n'),

            SubToken::ClosingLongBracket => {
                if sub_lexer.span().len() == open_len {
                    break;
                } else {
                    contents.extend_from_slice(sub_lexer.slice())
                }
            }
        }
    }

    lexer.bump(sub_lexer.span().end);
    Ok(Cow::Owned(contents))
}
