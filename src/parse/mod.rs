use {lalrpop_util::lalrpop_mod, thiserror::Error};

mod chunk_builder;
mod lex;

lalrpop_mod!(grammar, "/parse/grammar.rs");

pub type LexicalError = lex::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] LexicalError),
}
