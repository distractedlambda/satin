use {lalrpop_util::lalrpop_mod, std::result, thiserror::Error};

mod lex;

lalrpop_mod!(grammar, "/parse/grammar.rs");

pub type LexicalError = lex::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] LexicalError),
}
