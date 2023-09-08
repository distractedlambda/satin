use std::borrow::{Cow, Borrow};

use cranelift_entity::packed_option::PackedOption;

use crate::ir::{Value, BlockRef, Op};

use {
    super::{
        lex::{Numeral, Token},
        Error,
    },
    crate::ir::{Graph, ValueRef},
    lalrpop_util::ParseError,
    std::result,
};

pub struct ChunkBuilder<'a> {
    pub graph: &'a mut Graph,
    pub current_block: PackedOption<BlockRef>,
}

type Result<'source, T> = result::Result<T, ParseError<usize, Token<'source>, Error>>;

impl<'a> ChunkBuilder<'a> {
    pub fn lt_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn gt_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn le_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn ge_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn eq_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn ne_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn pow_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn concat_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn add_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn sub_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn mul_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn div_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn rem_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn floordiv_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn shl_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn shr_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn bitand_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn bitor_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn bitxor_expression(&mut self, lhs: ValueRef, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn not_expression(&mut self, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn len_expression(&mut self, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn bitnot_expression(&mut self, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn neg_expression(&mut self, rhs: ValueRef) -> ValueRef {
        todo!()
    }

    pub fn label(&mut self, name: &'a [u8]) -> Result<'a, ()> {
        todo!()
    }

    pub fn break_statement(&mut self) -> Result<'a, ()> {
        todo!()
    }

    pub fn goto_statement(&mut self, target: &'a [u8]) -> Result<'a, ()> {
        todo!()
    }

    pub fn parenthesized_expression(&mut self, inner: ValueRef)  -> ValueRef {
        todo!()
    }

    pub fn nil(&mut self) -> ValueRef {
        self.graph.values.intern(&Value::Nil)
    }

    pub fn bool(&mut self, value: bool) -> ValueRef {
        self.graph.values.intern(&Value::Bool(value))
    }

    pub fn numeral(&mut self, value: Numeral) -> ValueRef {
        self.graph.values.intern(&match value {
            Numeral::Int(v) => Value::Int(v),
            Numeral::Float(v) => Value::Float(v),
        })
    }

    pub fn string(&mut self, value: impl Borrow<[u8]>) -> ValueRef {
        let string = self.graph.strings.intern(value.borrow());
        self.graph.values.intern(&Value::String(string))
    }

    pub fn ellipses(&mut self) -> Result<'a, ValueRef> {
        todo!()
    }

    pub fn name_expression(&mut self, name: &'a [u8]) -> Result<'a, ValueRef> {
        todo!()
    }

    pub fn index(&mut self, table: ValueRef, key: ValueRef) -> ValueRef {
        let instr = self.graph.append_new_instruction(self.current_block.unwrap(), Op::Index(table, key));
        self.graph.values.intern(&Value::InstructionResult(instr))
    }

    pub fn call(&mut self, function: ValueRef, args: ValueRef) -> ValueRef {
        todo!()
    }
}
