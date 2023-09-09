use {
    ahash::AHashMap,
    cranelift_bforest::{Map, MapForest},
    cranelift_entity::{EntityList, EntityRef, ListPool, PrimaryMap},
    std::{borrow::Borrow, hash::Hash, sync::Arc},
};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}

id_type!(ContinuationRef);
id_type!(StringRef);
id_type!(ValueRef);

pub type ValueList = EntityList<ValueRef>;

pub type ValueMap = Map<ValueRef, ValueRef>;

#[derive(Clone)]
pub enum Op {
    Add,
    BitAnd,
    BitNot,
    BitOr,
    BitXor,
    Concat,
    Div,
    Eq,
    Floordiv,
    Ge,
    Gt,
    Index,
    Le,
    Length,
    LocalLoad,
    LocalStore,
    Lt,
    Mul,
    Ne,
    Neg,
    Newindex,
    NewLocal,
    NewTable,
    Not,
    Pow,
    Rem,
    Sub,
}

pub enum Continuation {
    Return,
    Perform(Op, ContinuationRef),
    Apply(ValueList, ContinuationRef),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Value {
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

impl<'a> From<&'a Value> for Value {
    fn from(value: &'a Value) -> Self {
        value.clone()
    }
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
    pub value_lists: ListPool<ValueRef>,
    pub value_maps: MapForest<ValueRef, ValueRef>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            strings: InterningMap::new(),
            values: InterningMap::new(),
            value_lists: ListPool::new(),
            value_maps: MapForest::new(),
        }
    }
}
