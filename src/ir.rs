use {
    crate::{entity_ref_type, lex::Numeral, string_pool::StringRef, vec_cell::VecCell},
    ahash::AHashMap,
    cranelift_bforest::{Set, SetForest},
    cranelift_entity::{packed_option::PackedOption, EntityList, ListPool, PrimaryMap},
    std::{
        borrow::BorrowMut,
        cell::{Cell, RefCell},
    },
};

entity_ref_type!(BlockRef);
entity_ref_type!(InstructionRef);
entity_ref_type!(ValueRef);

pub struct Graph {
    block_sets: SetForest<BlockRef>,
    blocks: PrimaryMap<BlockRef, Block>,
    instructions: PrimaryMap<InstructionRef, Instruction>,
    value_dedup: AHashMap<Value, ValueRef>,
    value_lists: ListPool<ValueRef>,
    values: PrimaryMap<ValueRef, Value>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            block_sets: SetForest::new(),
            blocks: PrimaryMap::new(),
            instructions: PrimaryMap::new(),
            value_dedup: AHashMap::new(),
            value_lists: ListPool::new(),
            values: PrimaryMap::new(),
        }
    }

    pub fn add_value(&mut self, value: Value) -> ValueRef {
        *self
            .value_dedup
            .entry(value)
            .or_insert_with(|| self.values.push(value))
    }

    pub fn new_block(&mut self) -> BlockRef {
        self.blocks.push(Block {
            head: None.into(),
            tail: None.into(),
            predecessors: Set::new(),
        })
    }

    pub fn new_instruction(&mut self, op: Op) -> InstructionRef {
        self.instructions.push(Instruction {
            op,
            prior: None.into(),
            next: None.into(),
            block: None.into(),
        })
    }

    pub fn new_value_list(&mut self, contents: &[ValueRef]) -> EntityList<ValueRef> {
        EntityList::from_slice(contents, &mut self.value_lists)
    }

    pub fn insert_at_end(&mut self, instruction: InstructionRef, block: BlockRef) {
        debug_assert!(self.instructions[instruction].prior.is_none());
        debug_assert!(self.instructions[instruction].next.is_none());
        debug_assert!(self.instructions[instruction].block.is_none());

        self.instructions[instruction].block = block.into();

        if let Some(tail) = self.blocks[block].tail.expand() {
            self.instructions[instruction].prior = tail.into();
            self.instructions[tail].next = instruction.into();
        } else {
            self.blocks[block].head = instruction.into();
        }

        self.blocks[block].tail = instruction.into();
    }

    pub fn append_instruction(&mut self, block: BlockRef, op: Op) -> InstructionRef {
        let instruction = self.new_instruction(op);
        self.insert_at_end(instruction, block);
        instruction
    }
}

pub struct Builder<'a> {
    graph: RefCell<&'a mut Graph>,
    current_block: Cell<PackedOption<BlockRef>>,
    expression_stack: VecCell<ValueRef>,
    merge_block_stack: VecCell<BlockRef>,
}

impl<'a> Builder<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
        Self {
            graph: RefCell::new(graph),
            current_block: Default::default(),
            expression_stack: Default::default(),
            merge_block_stack: Default::default(),
        }
    }

    pub fn build_not(&self) {
        let graph = &mut *self.graph.borrow_mut();
        let operand = self.expression_stack.pop().unwrap();
        self.expression_stack
            .push(graph.add_value(match graph.values[operand] {
                Value::Bool(true) | Value::Int(_) | Value::Float(_) | Value::String(_) => {
                    Value::Bool(false)
                }

                Value::Bool(false) | Value::Nil => Value::Bool(true),

                _ => Value::Not(operand),
            }));
    }

    pub fn build_len(&self) {
        todo!()
    }

    pub fn emit_unm(&self) {
        todo!()
    }

    pub fn build_bnot(self) {
        todo!()
    }

    pub fn build_add(&self) {
        todo!()
    }

    pub fn build_partial_and(&self) {
        let graph = &mut *self.graph.borrow_mut();
        let lhs = self.expression_stack.pop().unwrap();
        let rhs_block = graph.new_block();
        let merge_block = graph.new_block();
        let merge_args = graph.new_value_list(&[lhs]);

        graph.append_instruction(
            self.current_block.get().unwrap(),
            Op::BranchIf(
                lhs,
                BranchTarget::new(rhs_block, EntityList::new()),
                BranchTarget::new(merge_block, merge_args),
            ),
        );

        self.current_block.set(rhs_block.into());
        self.merge_block_stack.push(merge_block);
    }

    pub fn build_and(&self) {
        let graph = &mut *self.graph.borrow_mut();
        let rhs = self.expression_stack.pop().unwrap();
        let merge_block = self.merge_block_stack.pop().unwrap();
        let merge_args = graph.new_value_list(&[rhs]);

        graph.append_instruction(
            self.current_block.get().unwrap(),
            Op::Branch(BranchTarget::new(merge_block, merge_args)),
        );

        self.current_block.set(merge_block.into());

        let merged_value = graph.add_value(Value::BlockArgument(merge_block, 0));
        self.expression_stack.push(merged_value);
    }

    pub fn build_or_incomplete(&self) {
        todo!()
    }

    pub fn build_or(&self) {}
}

pub struct Block {
    head: PackedOption<InstructionRef>,
    tail: PackedOption<InstructionRef>,
    predecessors: Set<BlockRef>,
}

pub struct Instruction {
    op: Op,
    prior: PackedOption<InstructionRef>,
    next: PackedOption<InstructionRef>,
    block: PackedOption<BlockRef>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(u64),
    String(StringRef),
    InstructionResult(InstructionRef),
    BlockArgument(BlockRef, u32),
    Unpack(ValueRef, u32),
    Trailing(ValueRef, u32),
    Not(ValueRef),
    CoerceBool(ValueRef),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value.to_bits())
    }
}

impl From<StringRef> for Value {
    fn from(value: StringRef) -> Self {
        Self::String(value)
    }
}

impl From<InstructionRef> for Value {
    fn from(value: InstructionRef) -> Self {
        Self::InstructionResult(value)
    }
}

impl From<Numeral> for Value {
    fn from(value: Numeral) -> Self {
        match value {
            Numeral::Int(v) => Self::Int(v),
            Numeral::Float(v) => Self::Float(v),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BranchTarget {
    block: BlockRef,
    args: EntityList<ValueRef>,
}

impl BranchTarget {
    fn new(block: BlockRef, args: EntityList<ValueRef>) -> Self {
        Self { block, args }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Table,

    Local,
    LocalGet(ValueRef),
    LocalSet(ValueRef, ValueRef),

    Branch(BranchTarget),
    BranchIf(ValueRef, BranchTarget, BranchTarget),

    Bnot(ValueRef),
    Len(ValueRef),
    Unm(ValueRef),

    Add(ValueRef, ValueRef),
    Band(ValueRef, ValueRef),
    Bor(ValueRef, ValueRef),
    Bxor(ValueRef, ValueRef),
    Concat(ValueRef, ValueRef),
    Div(ValueRef, ValueRef),
    Eq(ValueRef, ValueRef),
    Ge(ValueRef, ValueRef),
    Gt(ValueRef, ValueRef),
    Idiv(ValueRef, ValueRef),
    Index(ValueRef, ValueRef),
    Le(ValueRef, ValueRef),
    Lt(ValueRef, ValueRef),
    Mod(ValueRef, ValueRef),
    Mul(ValueRef, ValueRef),
    Ne(ValueRef, ValueRef),
    Pow(ValueRef, ValueRef),
    Shl(ValueRef, ValueRef),
    Shr(ValueRef, ValueRef),
    Sub(ValueRef, ValueRef),

    Newindex(ValueRef, ValueRef, ValueRef),

    Call(ValueRef, EntityList<ValueRef>),
    TailCall(ValueRef, EntityList<ValueRef>),

    Return(EntityList<ValueRef>),
}
