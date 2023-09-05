use {
    ahash::AHashMap,
    cranelift_bforest::{Map, MapForest},
    cranelift_entity::{packed_option::PackedOption, EntityList, EntityRef, PrimaryMap},
    std::{borrow::Borrow, hash::Hash, sync::Arc},
};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}

id_type!(BlockRef);
id_type!(InstructionRef);
id_type!(StringRef);
id_type!(ValueRef);
id_type!(TypeRef);

/// An operation carried out by an [Instruction].
#[derive(Clone)]
pub enum Op {
    /// Constructs and initializes a new table, with semantics modeled after
    /// Lua's [table
    /// constructors][https://www.lua.org/manual/5.4/manual.html#3.4.9].
    ///
    /// Note that, unlike the Lua construct, this instruction cannot contain
    /// nested computation or control; it merely represents the final step of
    /// collecting already-evaluated field initializers into a table.
    NewTable {
        keyed_values: Map<ValueRef, ValueRef>,
        trailing_values: PackedOption<ValueRef>,
        trailing_values_start_index: i64,
    },

    /// Constructs a new `local` variable with an initial value. Since `local`s
    /// in Lua have "location semantics" (mutations are observed everywhere in
    /// the local scope, including within escaped closures), we model them as a
    /// distinct kind of mutable object holding a single value, where the IR
    /// only tracks that object, not its contained value.
    NewLocal { init: ValueRef },

    /// Reads the current value of a `local` variable.
    LocalLoad { local: ValueRef },

    /// Writes a new value to a `local` variable.
    LocalStore { local: ValueRef, value: ValueRef },

    /// Performs a Lua indexed assignment (`a[b] = c` or `a.b = c`, or `a = b`
    /// where `a` does not resolve to a `local`), possibly dispatching to a
    /// metamethod.
    Newindex {
        table: ValueRef,
        key: ValueRef,
        value: ValueRef,
    },

    /// Performs a Lua function call in non-tail position, possibly dispatching
    /// to a metamethod. The single `args` value represents the full pack of
    /// arguments to the function.
    Call { callee: ValueRef, args: ValueRef },

    /// Performs a Lua function call in tail position, possibly dispatching to a
    /// metamethod. The single `args` value represents the full pack of
    /// arguments to the function.
    TailCall { callee: ValueRef, args: ValueRef },

    /// Performs an unconditional branch to a [Block], potentially passing
    /// [Value]s for the block's arguments.
    Branch {
        target: BlockRef,
        args: EntityList<ValueRef>,
    },

    /// Performs a conditional branch to a [Block], potentially passing [Value]s
    /// for the block's arguments. The treatment of the `condition` follow's
    /// Lua's semantics (namely, `nil` and `false` are the only non-true
    /// values).
    BranchIf {
        target: BlockRef,
        condition: ValueRef,
        args: EntityList<ValueRef>,
    },

    /// Performs a `return` from the containing function.
    Return { values: ValueRef },
}

/// An operand to an [Instruction]. [Value]s may express computation by being
/// defined in terms of other [Value]s, but are always pure, and are prohibited
/// from forming cycles on their own.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    /// Refers to an indexed argument of a [Block].
    BlockArg(BlockRef, u32),

    /// Refers to the result of an [Instruction].
    InstructionResult(InstructionRef),

    /// Represents a sequence of zero or more values, with the same semantics as
    /// a comma-separated list of expressions in Lua.
    MultipleValues(EntityList<ValueRef>),

    /// A constant representing the Lua `nil` value.
    Nil,

    /// A constant representing a Lua `true` or `false` value.
    Bool(bool),

    /// A constant representing a Lua integer number.
    Int(i64),

    /// A constant representing a Lua floating-point number.
    Float(u64),

    /// A constant representing a Lua string.
    String(StringRef),
}

/// The type of a [Value].
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

/// A control-dependent operation, which may cause or observe side-effects.
/// Every [Instruction] exists within a [Block], in a linear order relative to
/// other [Instruction]s in that same [Block].
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

/// A [PrimaryMap] augmented with a hash table that deduplicates entities
/// according to their [Hash] and [Eq] implementations.
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
