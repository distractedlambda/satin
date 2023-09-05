use {
    crate::{
        ir::{BlockRef, Graph, Op, Value, ValueRef},
        lex::{Numeral, SpannedToken, Token, TokenStream},
    },
    cranelift_bforest::Map,
    cranelift_entity::packed_option::PackedOption,
    std::{ops::Range, result},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("lexical error: {0}")]
    LexicalError(#[from] crate::lex::Error),

    #[error("unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("unexpected token")]
    UnexpectedToken(Range<usize>),
}

struct Parser<'a> {
    graph: &'a mut Graph,
    token_stream: TokenStream<'a>,
    current_block: PackedOption<BlockRef>,
}

pub type Result<T> = result::Result<T, Error>;

impl<'a> Parser<'a> {
    fn new(graph: &'a mut Graph, source: &'a [u8]) -> Self {
        Self {
            graph,
            token_stream: TokenStream::new(source),
            current_block: None.into(),
        }
    }

    fn source(&self) -> &'a [u8] {
        self.token_stream.source()
    }

    fn next_token(&mut self) -> Result<SpannedToken> {
        Ok(self
            .token_stream
            .next()
            .ok_or(Error::UnexpectedEndOfInput)??)
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

    fn parse_block(&mut self) -> Result<()> {
        todo!()
    }

    fn parse_statement(&mut self) -> Result<bool> {
        let (token, span) = self.next_token()?;
        match token {
            Token::Semicolon => Ok(true),

            Token::KwBreak => todo!(),

            Token::KwGoto => {
                let (token, span) = self.next_token()?;
                if let Token::Name = token {
                    todo!()
                } else {
                    Err(Error::UnexpectedToken(span))
                }
            }

            Token::KwWhile => {
                let condition = self.parse_expression()?;

                let (token, span) = self.next_token()?;
                if !matches!(token, Token::KwDo) {
                    return Err(Error::UnexpectedToken(span));
                }

                self.parse_block()?;

                let (token, span) = self.next_token()?;
                if !matches!(token, Token::KwEnd) {
                    return Err(Error::UnexpectedToken(span));
                }

                Ok(true)
            }

            Token::KwRepeat => {
                self.parse_block()?;

                let (token, span) = self.next_token()?;
                if !matches!(token, Token::KwUntil) {
                    return Err(Error::UnexpectedToken(span))
                }

                let condition = self.parse_expression()?;

                todo!()
            }



            _ => Ok(false),
        }
    }

    fn parse_expression(&mut self) -> Result<ValueRef> {
        todo!()
    }

    fn parse_atomic_expression(&mut self) -> Result<ValueRef> {
        let (token, span) = self.next_token()?;

        match token {
            Token::Name => {
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
                let inner = self.parse_expression()?;
                let (token, span) = self.next_token()?;
                if let Token::RParen = token {
                    Ok(inner)
                } else {
                    Err(Error::UnexpectedToken(span))
                }
            }

            Token::LCurly => {
                let mut keyed_values = Map::new();
                let mut trailing_values = None.into();
                let mut trailing_values_start_index = 0;

                loop {
                    let (token, span) = self.next_token()?;

                    match token {
                        Token::RCurly => {
                            break;
                        }

                        Token::LSquare => {
                            let key = self.parse_expression()?;

                            let (token, span) = self.next_token()?;

                            if !matches!(token, Token::RSquare) {
                                return Err(Error::UnexpectedToken(span));
                            }

                            let (token, span) = self.next_token()?;

                            if !matches!(token, Token::Equals) {
                                return Err(Error::UnexpectedToken(span));
                            }

                            let value = self.parse_expression()?;

                            keyed_values.insert(key, value, &mut self.graph.value_maps, &());
                        }

                        Token::Name => {
                            let (eq_token, eq_span) = self.next_token()?;
                            if let Token::Equals = eq_token {
                                todo!()
                            } else {
                                self.token_stream.backtrack((eq_token, eq_span));
                                self.token_stream.backtrack((token, span));
                            }
                        }

                        _ => todo!(),
                    };

                    let (token, span) = self.next_token()?;

                    match token {
                        Token::Semicolon | Token::Comma => (),
                        Token::RCurly => break,
                        _ => return Err(Error::UnexpectedToken(span)),
                    }
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

            _ => return Err(Error::UnexpectedToken(span)),
        }
    }
}
