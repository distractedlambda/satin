use {
    super::{Error, Token},
    logos::{FilterResult, Lexer, Logos},
};

#[derive(Debug, Logos)]
#[logos(error = Error)]
enum SubToken {
    #[regex(b".+")]
    LiteralSegment,

    #[regex(br"\]=*\]")]
    ClosingLongBracket,
}

pub fn callback<T>(lexer: &mut Lexer<Token>) -> FilterResult<T, Error> {
    let open_len = lexer.span().len() - 2;
    let mut sub_lexer = SubToken::lexer(lexer.remainder());
    loop {
        match sub_lexer.next() {
            None => return FilterResult::Error(Error::UnclosedLongComment),

            Some(Err(e)) => return FilterResult::Error(e),

            Some(Ok(SubToken::LiteralSegment)) => (),

            Some(Ok(SubToken::ClosingLongBracket)) => {
                if sub_lexer.span().len() == open_len {
                    lexer.bump(sub_lexer.span().end);
                    return FilterResult::Skip;
                }
            }
        }
    }
}
