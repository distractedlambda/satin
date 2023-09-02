use cranelift_entity::{entity_impl, EntityList, PrimaryMap, packed_option::PackedOption};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}

id_type!(BlockRef);
id_type!(LocalRef);
id_type!(InstructionRef);

enum Op {
    NewTable,

    NilConstant,
    FalseConstant,
    TrueConstant,
    IntConstant(i64),
    FloatConstant(f64),
    LightUserdataConstant(u64),

    Bnot(InstructionRef),
    Len(InstructionRef),
    Unm(InstructionRef),
    CoerceToBool(InstructionRef),
    CoerceToSingleValue(InstructionRef),

    Add(InstructionRef, InstructionRef),
    Band(InstructionRef, InstructionRef),
    Bor(InstructionRef, InstructionRef),
    Bxor(InstructionRef, InstructionRef),
    Concat(InstructionRef, InstructionRef),
    Div(InstructionRef, InstructionRef),
    Eq(InstructionRef, InstructionRef),
    Ge(InstructionRef, InstructionRef),
    Gt(InstructionRef, InstructionRef),
    Idiv(InstructionRef, InstructionRef),
    Index(InstructionRef, InstructionRef),
    Le(InstructionRef, InstructionRef),
    Lt(InstructionRef, InstructionRef),
    Mod(InstructionRef, InstructionRef),
    Mul(InstructionRef, InstructionRef),
    Ne(InstructionRef, InstructionRef),
    Pow(InstructionRef, InstructionRef),
    Shl(InstructionRef, InstructionRef),
    Shr(InstructionRef, InstructionRef),
    Sub(InstructionRef, InstructionRef),

    Newindex(InstructionRef, InstructionRef, InstructionRef),

    Call(InstructionRef, EntityList<InstructionRef>),
    TailCall(InstructionRef, EntityList<InstructionRef>),

    Return(EntityList<InstructionRef>),

    Branch(BlockRef),

    BranchIf(BlockRef, InstructionRef),

    GetLocal(LocalRef),

    SetLocal(LocalRef, InstructionRef),

    Raise(InstructionRef),
}

struct Instruction {
    op: Op,
    block: PackedOption<InstructionRef>,
    next: PackedOption<InstructionRef>,
    prior: PackedOption<InstructionRef>,
}

struct Graph {
    instructions: PrimaryMap<InstructionRef, Instruction>,
}
