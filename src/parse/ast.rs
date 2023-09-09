use {
    super::StringRef,
    cranelift_bforest::{Map, MapForest},
    cranelift_entity::{packed_option::PackedOption, EntityList, ListPool, PrimaryMap},
    std::{
        cell::{Cell, UnsafeCell},
        ops::Index,
    },
};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}

id_type!(AssignmentTargetRef);
id_type!(BlockRef);
id_type!(ExpressionRef);
id_type!(FieldRef);
id_type!(FunctionRef);
id_type!(GenericForLoopRef);
id_type!(LabelRef);
id_type!(LocalRef);
id_type!(NumericalForLoopRef);
id_type!(StatementRef);

pub struct Ast {
    assignment_target_lists: ListPool<AssignmentTargetRef>,
    assignment_targets: PrimaryMap<AssignmentTargetRef, AssignmentTarget>,
    block_lists: ListPool<BlockRef>,
    blocks: PrimaryMap<BlockRef, Block>,
    expression_lists: ListPool<ExpressionRef>,
    expressions: PrimaryMap<ExpressionRef, Expression>,
    field_lists: ListPool<FieldRef>,
    fields: PrimaryMap<FieldRef, Field>,
    functions: PrimaryMap<FunctionRef, Function>,
    generic_for_loops: PrimaryMap<GenericForLoopRef, GenericForLoop>,
    label_maps: MapForest<StringRef, LabelRef>,
    labels: PrimaryMap<LabelRef, Label>,
    local_maps: MapForest<StringRef, LocalRef>,
    locals: PrimaryMap<LocalRef, Local>,
    numerical_for_loops: PrimaryMap<NumericalForLoopRef, NumericalForLoop>,
    statement_lists: ListPool<StatementRef>,
    statements: PrimaryMap<StatementRef, Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            assignment_target_lists: ListPool::new(),
            assignment_targets: PrimaryMap::new(),
            block_lists: ListPool::new(),
            blocks: PrimaryMap::new(),
            expression_lists: ListPool::new(),
            expressions: PrimaryMap::new(),
            field_lists: ListPool::new(),
            fields: PrimaryMap::new(),
            functions: PrimaryMap::new(),
            generic_for_loops: PrimaryMap::new(),
            label_maps: MapForest::new(),
            labels: PrimaryMap::new(),
            local_maps: MapForest::new(),
            locals: PrimaryMap::new(),
            numerical_for_loops: PrimaryMap::new(),
            statement_lists: ListPool::new(),
            statements: PrimaryMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.assignment_target_lists.clear();
        self.assignment_targets.clear();
        self.block_lists.clear();
        self.blocks.clear();
        self.expression_lists.clear();
        self.expressions.clear();
        self.field_lists.clear();
        self.fields.clear();
        self.functions.clear();
        self.generic_for_loops.clear();
        self.label_maps.clear();
        self.labels.clear();
        self.local_maps.clear();
        self.locals.clear();
        self.numerical_for_loops.clear();
        self.statement_lists.clear();
        self.statements.clear();
    }
}

impl Index<AssignmentTargetRef> for Ast {
    type Output = AssignmentTarget;

    fn index(&self, index: AssignmentTargetRef) -> &Self::Output {
        &self.assignment_targets[index]
    }
}

impl Index<EntityList<AssignmentTargetRef>> for Ast {
    type Output = [AssignmentTargetRef];

    fn index(&self, index: EntityList<AssignmentTargetRef>) -> &Self::Output {
        index.as_slice(&self.assignment_target_lists)
    }
}

impl Index<BlockRef> for Ast {
    type Output = Block;

    fn index(&self, index: BlockRef) -> &Self::Output {
        &self.blocks[index]
    }
}

impl Index<EntityList<BlockRef>> for Ast {
    type Output = [BlockRef];

    fn index(&self, index: EntityList<BlockRef>) -> &Self::Output {
        index.as_slice(&self.block_lists)
    }
}

impl Index<ExpressionRef> for Ast {
    type Output = Expression;

    fn index(&self, index: ExpressionRef) -> &Self::Output {
        &self.expressions[index]
    }
}

impl Index<EntityList<ExpressionRef>> for Ast {
    type Output = [ExpressionRef];

    fn index(&self, index: EntityList<ExpressionRef>) -> &Self::Output {
        index.as_slice(&self.expression_lists)
    }
}

impl Index<FieldRef> for Ast {
    type Output = Field;

    fn index(&self, index: FieldRef) -> &Self::Output {
        &self.fields[index]
    }
}

impl Index<EntityList<FieldRef>> for Ast {
    type Output = [FieldRef];

    fn index(&self, index: EntityList<FieldRef>) -> &Self::Output {
        index.as_slice(&self.field_lists)
    }
}

impl Index<FunctionRef> for Ast {
    type Output = Function;

    fn index(&self, index: FunctionRef) -> &Self::Output {
        &self.functions[index]
    }
}

impl Index<GenericForLoopRef> for Ast {
    type Output = GenericForLoop;

    fn index(&self, index: GenericForLoopRef) -> &Self::Output {
        &self.generic_for_loops[index]
    }
}

impl Index<LabelRef> for Ast {
    type Output = Label;

    fn index(&self, index: LabelRef) -> &Self::Output {
        &self.labels[index]
    }
}

impl Index<LocalRef> for Ast {
    type Output = Local;

    fn index(&self, index: LocalRef) -> &Self::Output {
        &self.locals[index]
    }
}

impl Index<NumericalForLoopRef> for Ast {
    type Output = NumericalForLoop;

    fn index(&self, index: NumericalForLoopRef) -> &Self::Output {
        &self.numerical_for_loops[index]
    }
}

impl Index<StatementRef> for Ast {
    type Output = Statement;

    fn index(&self, index: StatementRef) -> &Self::Output {
        &self.statements[index]
    }
}

impl Index<EntityList<StatementRef>> for Ast {
    type Output = [StatementRef];

    fn index(&self, index: EntityList<StatementRef>) -> &Self::Output {
        index.as_slice(&self.statement_lists)
    }
}

pub struct Builder<'a> {
    ast: UnsafeCell<&'a mut Ast>,
    current_block: Cell<PackedOption<BlockRef>>,
}

impl<'a> Builder<'a> {
    pub fn new(ast: &'a mut Ast) -> Self {
        Self {
            ast: UnsafeCell::new(ast),
            current_block: Cell::new(None.into()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AssignmentTarget {
    Local(LocalRef),
    Newindex(ExpressionRef, ExpressionRef),
}

#[derive(Clone)]
pub struct Block {
    parent: PackedOption<BlockRef>,
    locals: Map<StringRef, LocalRef>,
    statements: EntityList<StatementRef>,
    returns: Option<EntityList<ExpressionRef>>,
}

#[derive(Clone, Copy)]
pub enum Expression {
    Add(ExpressionRef, ExpressionRef),
    And(ExpressionRef, ExpressionRef),
    Band(ExpressionRef, ExpressionRef),
    Bnot(ExpressionRef),
    Bor(ExpressionRef, ExpressionRef),
    Bxor(ExpressionRef, ExpressionRef),
    Call(ExpressionRef, EntityList<ExpressionRef>),
    Concat(ExpressionRef, ExpressionRef),
    Div(ExpressionRef, ExpressionRef),
    Ellipses,
    Eq(ExpressionRef, ExpressionRef),
    False,
    Float(u64),
    Function(FunctionRef),
    Ge(ExpressionRef, ExpressionRef),
    Gt(ExpressionRef, ExpressionRef),
    Idiv(ExpressionRef, ExpressionRef),
    Index(ExpressionRef, ExpressionRef),
    Int(i64),
    Le(ExpressionRef, ExpressionRef),
    Len(ExpressionRef),
    Local(LocalRef),
    Lt(ExpressionRef, ExpressionRef),
    Mod(ExpressionRef, ExpressionRef),
    Mul(ExpressionRef, ExpressionRef),
    Ne(ExpressionRef, ExpressionRef),
    Nil,
    Not(ExpressionRef),
    Or(ExpressionRef, ExpressionRef),
    Pow(ExpressionRef, ExpressionRef),
    Shl(ExpressionRef, ExpressionRef),
    String(StringRef),
    Sub(ExpressionRef, ExpressionRef),
    Table(EntityList<FieldRef>),
    True,
    Unm(ExpressionRef),
}

#[derive(Clone, Copy)]
pub enum Field {
    Keyed(ExpressionRef, ExpressionRef),
    Ordinal(ExpressionRef),
}

#[derive(Clone, Copy)]
pub struct Function {
    body: BlockRef,
}

#[derive(Clone, Copy)]
pub struct GenericForLoop {
    vars: EntityList<LocalRef>,
    iterator: EntityList<ExpressionRef>,
    body: BlockRef,
}

#[derive(Clone, Copy)]
pub struct Label {
    name: StringRef,
}

#[derive(Clone, Copy)]
pub struct Local {
    name: StringRef,
    attribute: PackedOption<StringRef>,
}

#[derive(Clone, Copy)]
pub struct NumericalForLoop {
    var: LocalRef,
    start: ExpressionRef,
    stop: ExpressionRef,
    step: PackedOption<ExpressionRef>,
    body: BlockRef,
}

#[derive(Clone, Copy)]
pub enum Statement {
    Assign(EntityList<AssignmentTargetRef>, EntityList<ExpressionRef>),
    Break,
    Call(ExpressionRef, EntityList<ExpressionRef>),
    Do(BlockRef),
    GenericForLoop(GenericForLoopRef),
    Goto(StringRef),
    If(EntityList<ExpressionRef>, EntityList<BlockRef>),
    Label(StringRef),
    DeclareLocal(LocalRef),
    NumericalForLoop(NumericalForLoopRef),
    RepeatUntil(BlockRef, ExpressionRef),
    While(ExpressionRef, BlockRef),
}
