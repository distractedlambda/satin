use cranelift_bforest::{MapForest, Map};

use {
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
id_type!(StringRef);
id_type!(ValueRef);
id_type!(TypeRef);

#[derive(Clone)]
pub enum Op {
    NewTable {
        keyed_values: Map<ValueRef, ValueRef>,
        trailing_values: PackedOption<ValueRef>,
        trailing_values_start_index: i64,
    },

    NewLocal,

    Newindex {
        table: ValueRef,
        key: ValueRef,
        value: ValueRef,
    },

    LocalLoad {
        local: ValueRef,
    },

    LocalStore {
        local: ValueRef,
        value: ValueRef,
    },

    Call {
        callee: ValueRef,
        args: ValueRef,
    },

    TailCall {
        callee: ValueRef,
        args: ValueRef,
    },

    Branch {
        target: BlockRef,
        args: EntityList<ValueRef>,
    },

    BranchIf {
        target: BlockRef,
        condition: ValueRef,
        args: EntityList<ValueRef>,
    },

    Return {
        values: ValueRef,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    BlockArg(BlockRef, u32),
    InstructionResult(InstructionRef),

    EmptyDynamicPack,

    DynamicNil,
    DynamicBool(bool),
    DynamicInt(i64),
    DynamicFloat(u64),
    DynamicString(StringRef),

    Bool(bool),
    I64(i64),
    F64(u64),
    String(StringRef),

    BoolToDynamic(ValueRef),
    I64ToDynamic(ValueRef),
    F64ToDynamic(ValueRef),
    StringToDynamic(ValueRef),
    TableToDynamic(ValueRef),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Dynamic,
    DynamicPack,
    Local,
    RawBool,
    RawI64,
    RawF64,
    RawString,
    Tuple(EntityList<TypeRef>),
}

impl<'a> From<&'a Value> for Value {
    fn from(value: &'a Value) -> Self {
        value.clone()
    }
}

#[derive(Clone)]
pub struct Instruction {
    op: Op,
    block: PackedOption<BlockRef>,
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
    pub value_maps: MapForest<ValueRef, ValueRef>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            strings: InterningMap::new(),
            values: InterningMap::new(),
            instructions: PrimaryMap::new(),
            blocks: PrimaryMap::new(),
            value_maps: MapForest::new(),
        }
    }

    pub fn new_block(&mut self) -> BlockRef {
        self.blocks.push(Block {
            head: None.into(),
            tail: None.into(),
        })
    }

    pub fn new_instruction(&mut self, op: Op) -> InstructionRef {
        self.instructions.push(Instruction {
            op,
            block: None.into(),
            next: None.into(),
            prior: None.into(),
        })
    }

    pub fn remove_instruction(&mut self, instruction: InstructionRef) {
        let instruction_data = &mut self.instructions[instruction];

        let (block, next, prior) = (
            instruction_data.block.take(),
            instruction_data.next.take(),
            instruction_data.prior.take(),
        );

        if let Some(block) = block {
            if let Some(next) = next {
                self.instructions[next].prior = prior.into();
            } else {
                self.blocks[block].tail = prior.into();
            }

            if let Some(prior) = prior {
                self.instructions[prior].next = next.into();
            } else {
                self.blocks[block].head = next.into();
            }
        } else {
            debug_assert!(next.is_none());
            debug_assert!(prior.is_none());
        }
    }

    pub fn append_instruction(&mut self, block: BlockRef, instruction: InstructionRef) {
        let block_data = &mut self.blocks[block];
        let instruction_data = &mut self.instructions[instruction];

        debug_assert!(instruction_data.block.is_none());
        debug_assert!(instruction_data.next.is_none());
        debug_assert!(instruction_data.prior.is_none());

        instruction_data.block = block.into();

        if let Some(prior) = block_data.tail.expand() {
            instruction_data.prior = prior.into();
            self.instructions[prior].next = instruction.into();
        } else {
            block_data.head = instruction.into();
        }

        block_data.tail = instruction.into();
    }

    pub fn append_new_instruction(&mut self, block: BlockRef, op: Op) -> InstructionRef {
        let instruction = self.new_instruction(op);
        self.append_instruction(block, instruction);
        instruction
    }
}
