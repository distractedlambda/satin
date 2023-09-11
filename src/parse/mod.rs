use {crate::lex, lalrpop_util::lalrpop_mod, std::result, thiserror::Error};

// lalrpop_mod!(grammar, "/parse/parser.rs");

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] lex::Error),
}
