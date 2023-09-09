use {
    super::{Error, Result, Token},
    crate::parse::StringRef,
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

pub fn callback(lexer: &mut Lexer<Token>) -> Result<StringRef> {
    let open_len = lexer.span().len();

    if matches!(lexer.slice(), &[b'\n', ..]) {
        lexer.bump(1);
    }

    lexer.extras.string_buffer.clear();
    let mut sub_lexer = SubToken::lexer(lexer.remainder());

    loop {
        match sub_lexer.next().ok_or(Error::UnclosedLongLiteral)?? {
            SubToken::LiteralSegment => lexer
                .extras
                .string_buffer
                .extend_from_slice(sub_lexer.slice()),
            SubToken::LineEnding => lexer.extras.string_buffer.push(b'\n'),

            SubToken::ClosingLongBracket => {
                if sub_lexer.span().len() == open_len {
                    break;
                } else {
                    lexer
                        .extras
                        .string_buffer
                        .extend_from_slice(sub_lexer.slice())
                }
            }
        }
    }

    lexer.bump(sub_lexer.span().end);
    Ok(lexer.extras.strings.intern(&lexer.extras.string_buffer))
}
