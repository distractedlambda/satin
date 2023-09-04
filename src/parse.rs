use {
    crate::{
        ir::{BlockRef, Graph, Op, StringRef, Value, ValueRef},
        lex::{Numeral, Token},
    },
    cranelift_bforest::Map,
    cranelift_entity::packed_option::PackedOption,
    logos::{Lexer, Logos},
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
    lexer: Lexer<'a, Token>,
    current_token: Option<Token>,
    current_block: PackedOption<BlockRef>,
}

pub type Result<T> = result::Result<T, Error>;

impl<'a> Parser<'a> {
    fn new(graph: &'a mut Graph, source: &'a [u8]) -> Self {
        Self {
            graph,
            lexer: Token::lexer(source),
            current_token: None,
            current_block: None.into(),
        }
    }

    fn maybe_current_token(&mut self) -> Result<Option<Token>> {
        Ok(if let Some(token) = self.current_token {
            Some(token)
        } else if let Some(token) = self.lexer.next() {
            let token = token?;
            self.current_token = Some(token);
            Some(token)
        } else {
            None
        })
    }

    fn current_token(&mut self) -> Result<Token> {
        if let Some(token) = self.maybe_current_token()? {
            Ok(token)
        } else {
            Err(Error::UnexpectedEndOfInput)
        }
    }

    fn consume_token(&mut self) {
        self.current_token.take().unwrap();
    }

    fn intern_name(&mut self) -> StringRef {
        self.graph.strings.intern(self.lexer.slice())
    }

    fn intern_string(&mut self) -> StringRef {
        self.graph
            .strings
            .intern(&self.lexer.extras.string_contents)
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
        match self.current_token()? {
            Token::Name => {
                let name_string = self.intern_name();
                self.consume_token();
                todo!()
            }

            Token::String => {
                let string = self.intern_string();
                self.consume_token();
                Ok(self.emit_value(Value::DynamicString(string)))
            }

            Token::Numeral(Numeral::Int(value)) => {
                self.consume_token();
                Ok(self.emit_value(Value::DynamicInt(value)))
            }

            Token::Numeral(Numeral::Float(value)) => {
                self.consume_token();
                Ok(self.emit_value(Value::DynamicFloat(value)))
            }

            Token::KwFalse => {
                self.consume_token();
                Ok(self.emit_value(Value::DynamicBool(false)))
            }

            Token::KwFunction => todo!(),

            Token::KwNil => {
                self.consume_token();
                Ok(self.emit_value(Value::DynamicNil))
            }

            Token::KwTrue => {
                self.consume_token();
                Ok(self.emit_value(Value::DynamicBool(true)))
            }

            Token::LParen => {
                self.consume_token();
                let inner = self.parse_expression()?;
                if let Token::RParen = self.current_token()? {
                    self.consume_token();
                    Ok(inner)
                } else {
                    Err(Error::UnexpectedToken)
                }
            }

            Token::LCurly => {
                self.consume_token();

                let mut keyed_values = Map::new();
                let mut trailing_values = None.into();
                let mut trailing_values_start_index = 0;

                loop {
                    match self.current_token()? {
                        Token::Name => todo!(),
                        Token::String => todo!(),
                        Token::Numeral(_) => todo!(),
                        Token::KwAnd => todo!(),
                        Token::KwBreak => todo!(),
                        Token::KwDo => todo!(),
                        Token::KwElse => todo!(),
                        Token::KwElseif => todo!(),
                        Token::KwEnd => todo!(),
                        Token::KwFalse => todo!(),
                        Token::KwFor => todo!(),
                        Token::KwFunction => todo!(),
                        Token::KwGoto => todo!(),
                        Token::KwIf => todo!(),
                        Token::KwIn => todo!(),
                        Token::KwLocal => todo!(),
                        Token::KwNil => todo!(),
                        Token::KwNot => todo!(),
                        Token::KwOr => todo!(),
                        Token::KwRepeat => todo!(),
                        Token::KwReturn => todo!(),
                        Token::KwThen => todo!(),
                        Token::KwTrue => todo!(),
                        Token::KwUntil => todo!(),
                        Token::KwWhile => todo!(),
                        Token::Plus => todo!(),
                        Token::Minus => todo!(),
                        Token::Star => todo!(),
                        Token::Slash => todo!(),
                        Token::Percent => todo!(),
                        Token::Caret => todo!(),
                        Token::Hash => todo!(),
                        Token::Ampersand => todo!(),
                        Token::Tilde => todo!(),
                        Token::Pipe => todo!(),
                        Token::LAngle2 => todo!(),
                        Token::RAngle2 => todo!(),
                        Token::Slash2 => todo!(),
                        Token::Equals2 => todo!(),
                        Token::TildeEquals => todo!(),
                        Token::LAngleEquals => todo!(),
                        Token::RAngleEquals => todo!(),
                        Token::LAngle => todo!(),
                        Token::RAngle => todo!(),
                        Token::Equals => todo!(),
                        Token::LParen => todo!(),
                        Token::RParen => todo!(),
                        Token::LCurly => todo!(),
                        Token::RCurly => todo!(),
                        Token::LSquare => todo!(),
                        Token::RSquare => todo!(),
                        Token::Colon2 => todo!(),
                        Token::Semicolon => todo!(),
                        Token::Colon => todo!(),
                        Token::Comma => todo!(),
                        Token::Dot => todo!(),
                        Token::Dot2 => todo!(),
                        Token::Dot3 => todo!(),
                    }
                }

                Ok(self.emit_instruction(Op::NewTable {
                    keyed_values,
                    trailing_values,
                    trailing_values_start_index,
                }))
            }

            Token::Dot3 => {
                self.consume_token();
                todo!()
            }

            _ => return Err(Error::UnexpectedToken),
        }
    }
}
