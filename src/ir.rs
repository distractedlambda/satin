use {
    ahash::AHashMap,
    cranelift_bforest::Set,
    cranelift_entity::{packed_option::PackedOption, EntityList, PrimaryMap},
    std::{ops::Range, path::PathBuf, sync::Arc},
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
id_type!(LocalRef);
id_type!(SourceFileRef);
id_type!(StringRef);

pub enum Op {
    NewTable,

    NilConstant,
    FalseConstant,
    TrueConstant,
    StringConstant(StringRef),
    IntConstant(i64),
    FloatConstant(f64),

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

pub struct Instruction {
    op: Op,
    block: PackedOption<InstructionRef>,
    next: PackedOption<InstructionRef>,
    prior: PackedOption<InstructionRef>,
    source_file: PackedOption<SourceFileRef>,
    source_span: Range<u32>,
}

pub struct Block {
    head: PackedOption<InstructionRef>,
    tail: PackedOption<InstructionRef>,
}

pub enum SourceFile {
    Path(PathBuf),
}

pub struct Local;

pub struct Graph {
    source_files: PrimaryMap<SourceFileRef, SourceFile>,

    strings: PrimaryMap<StringRef, Arc<[u8]>>,
    string_dedup_table: AHashMap<Arc<[u8]>, StringRef>,

    instructions: PrimaryMap<InstructionRef, Instruction>,

    locals: PrimaryMap<LocalRef, Local>,

    blocks: PrimaryMap<BlockRef, Block>,
}

impl Graph {
    pub fn add_source_file(&mut self, sf: SourceFile) -> SourceFileRef {
        self.source_files.push(sf)
    }

    pub fn source_File(&self, key: SourceFileRef) -> &SourceFile {
        &self.source_files[key]
    }

    pub fn add_string(&mut self, contents: &[u8]) -> StringRef {
        if let Some(&sr) = self.string_dedup_table.get(contents) {
            return sr;
        }

        let string: Arc<[u8]> = contents.into();
        let sr = self.strings.push(string.clone());
        let _ = self.string_dedup_table.insert(string, sr);
        sr
    }

    pub fn string(&self, key: StringRef) -> &Arc<[u8]> {
        &self.strings[key]
    }

    pub fn instruction(&self, key: InstructionRef) -> &Instruction {
        &self.instructions[key]
    }

    pub fn local(&self, key: LocalRef) -> &Local {
        &self.locals[key]
    }
}
