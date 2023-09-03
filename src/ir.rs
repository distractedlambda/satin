use {
    crate::lex::Numeral,
    ahash::AHashMap,
    cranelift_entity::{packed_option::PackedOption, EntityList, EntityRef, PrimaryMap},
    std::{borrow::Borrow, hash::Hash, sync::Arc},
};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}

id_type!(BlockRef);
id_type!(InstructionRef);
id_type!(SourceFileRef);
id_type!(StringRef);
id_type!(ValueRef);

#[derive(Clone, Copy, Debug)]
pub enum Op {
    NewTable,
    NewLocal,
    LocalLoad(ValueRef),
    LocalStore(ValueRef, ValueRef),
    Call(ValueRef, MultipleValues, BlockRef),
    TailCall(ValueRef, MultipleValues),
    Branch(BlockRef, MultipleValues),
    BranchIf(ValueRef, BlockRef, MultipleValues),
    Return(MultipleValues),
}

#[derive(Clone, Copy, Debug)]
pub struct MultipleValues {
    formals: EntityList<ValueRef>,
    trailing_source: PackedOption<BlockRef>,
    trailing_start: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(u64),
    String(StringRef),
    BlockArg(BlockRef, u32),
    InstructionResult(InstructionRef),
}

impl From<Numeral> for Value {
    fn from(value: Numeral) -> Self {
        match value {
            Numeral::Int(v) => Value::Int(v),
            Numeral::Float(v) => Value::Float(v.to_bits()),
        }
    }
}

impl<'a> From<&'a Value> for Value {
    fn from(value: &'a Value) -> Self {
        value.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    op: Op,
    block: PackedOption<InstructionRef>,
    next: PackedOption<InstructionRef>,
    prior: PackedOption<InstructionRef>,
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    head: PackedOption<InstructionRef>,
    tail: PackedOption<InstructionRef>,
}

#[derive(Clone, Debug)]
pub struct InterningMap<K: EntityRef, V>(PrimaryMap<K, V>, AHashMap<V, K>);

impl<K: EntityRef, V> InterningMap<K, V> {
    pub fn new() -> Self {
        Self(PrimaryMap::new(), AHashMap::new())
    }

    pub fn get(&self, key: K) -> &V {
        &self.0[key]
    }

    pub fn intern<Q>(&mut self, value: &Q) -> K
    where
        Q: Eq + Hash + ?Sized,
        V: Borrow<Q> + Clone + Eq + for<'a> From<&'a Q> + Hash,
    {
        if let Some(&key) = self.1.get(value) {
            return key;
        }

        let value: V = value.into();
        let key = self.0.push(value.clone());
        self.1.insert(value, key);
        key
    }
}

pub struct Graph {
    pub strings: InterningMap<StringRef, Arc<[u8]>>,
    pub values: InterningMap<ValueRef, Value>,
    pub instructions: PrimaryMap<InstructionRef, Instruction>,
    pub blocks: PrimaryMap<BlockRef, Block>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            strings: InterningMap::new(),
            values: InterningMap::new(),
            instructions: PrimaryMap::new(),
            blocks: PrimaryMap::new(),
        }
    }
}
