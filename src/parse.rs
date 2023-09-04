use {
    crate::{
        ir::{BlockRef, Graph, Op, Value, ValueRef},
        lex::{Numeral, SpannedToken, Token, TokenStream},
    },
    cranelift_bforest::Map,
    cranelift_entity::packed_option::PackedOption,
    std::result,
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] crate::lex::Error),

    #[error("unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("unexpected token")]
    UnexpectedToken,
}

struct Parser<'a> {
    graph: &'a mut Graph,
    current_block: PackedOption<BlockRef>,
    token_stream: TokenStream<'a>,
}

pub type Result<T> = result::Result<T, Error>;

impl<'a> Parser<'a> {
    fn new(graph: &'a mut Graph, source: &'a [u8]) -> Self {
        Self {
            graph,
            current_block: None.into(),
            token_stream: TokenStream::new(source),
        }
    }

    fn source(&self) -> &'a [u8] {
        self.token_stream.source()
    }

    fn parse_expression(&mut self) -> Result<ValueRef> {
        todo!()
    }

    fn emit_value(&mut self, value: Value) -> ValueRef {
        self.graph.values.intern(&value)
    }

    fn emit_instruction(&mut self, op: Op) -> ValueRef {
        let instruction = self
            .graph
            .append_new_instruction(self.current_block.unwrap(), op);

        self.emit_value(Value::InstructionResult(instruction))
    }

    fn parse_atomic_expression(&mut self) -> Result<ValueRef> {
        let SpannedToken {
            ref token,
            ref span,
        } = self
            .token_stream
            .lookahead(1)?
            .ok_or(Error::UnexpectedEndOfInput)?;

        match token {
            Token::Name => {
                self.token_stream.advance(1);
                todo!()
            }

            Token::String(_) => {
                todo!()
            }

            Token::Numeral(Numeral::Int(value)) => {
                todo!()
            }

            Token::Numeral(Numeral::Float(value)) => {
                todo!()
            }

            Token::KwFalse => {
                todo!()
            }

            Token::KwFunction => todo!(),

            Token::KwNil => {
                todo!()
            }

            Token::KwTrue => {
                todo!()
            }

            Token::LParen => {
                self.token_stream.advance(1);
                let inner = self.parse_expression()?;
                todo!()
            }

            Token::LCurly => {
                self.token_stream.advance(1);

                let mut keyed_values = Map::new();
                let mut trailing_values = None.into();
                let mut trailing_values_start_index = 0;

                loop {
                    todo!()
                }

                Ok(self.emit_instruction(Op::NewTable {
                    keyed_values,
                    trailing_values,
                    trailing_values_start_index,
                }))
            }

            Token::Dot3 => {
                todo!()
            }

            _ => return Err(Error::UnexpectedToken),
        }
    }
}
