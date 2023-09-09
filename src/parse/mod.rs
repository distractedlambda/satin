pub use string_pool::{StringPool, StringRef};
use {lalrpop_util::lalrpop_mod, lex::Token, std::result, thiserror::Error};

mod ast;
mod lex;
mod string_pool;

lalrpop_mod!(grammar, "/parse/parser.rs");

pub type LexicalError = lex::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] LexicalError),
}
