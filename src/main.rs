#![feature(const_option)]
#![feature(split_array)]

use std::ptr::NonNull;

mod ir;
mod lex;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    False,
    Nil,

    True,

    Float,
    Int,
    LightUserdata,

    FullUserdata,
    Function,
    String,
    Table,
    Thread,
}

impl Type {
    fn is_truthy(self) -> bool {
        match self {
            Self::False | Self::Nil => false,
            _ => true,
        }
    }

    fn is_empty(self) -> bool {
        match self {
            Self::False | Self::Nil | Self::True => true,
            _ => false,
        }
    }

    fn is_object(self) -> bool {
        match self {
            Self::FullUserdata | Self::Function | Self::String | Self::Table | Self::Thread => true,
            _ => false,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union ValueRepr {
    empty: (),
    float: f64,
    full_userdata: NonNull<FullUserdata>,
    function: NonNull<Function>,
    int: i64,
    light_userdata: usize,
    string: NonNull<String>,
    table: NonNull<Table>,
    thread: NonNull<Thread>,
}

#[derive(Clone, Copy, Debug)]
enum Value {
    False,
    Nil,
    True,
    Float(f64),
    Int(i64),
    LightUserdata(usize),
    FullUserdata(NonNull<FullUserdata>),
    Function(NonNull<Function>),
    String(NonNull<String>),
    Table(NonNull<Table>),
    Thread(NonNull<Thread>),
}

impl Value {
    unsafe fn from_raw(typ: Type, repr: ValueRepr) -> Self {
        match typ {
            Type::False => Self::False,
            Type::Nil => Self::Nil,
            Type::True => Self::True,
            Type::Float => Self::Float(repr.float),
            Type::Int => Self::Int(repr.int),
            Type::LightUserdata => Self::LightUserdata(repr.light_userdata),
            Type::FullUserdata => Self::FullUserdata(repr.full_userdata),
            Type::Function => Self::Function(repr.function),
            Type::String => Self::String(repr.string),
            Type::Table => Self::Table(repr.table),
            Type::Thread => Self::Thread(repr.thread),
        }
    }

    fn to_raw(self) -> (Type, ValueRepr) {
        match self {
            Self::False => (Type::False, ValueRepr { empty: () }),
            Self::Nil => (Type::Nil, ValueRepr { empty: () }),
            Self::True => (Type::True, ValueRepr { empty: () }),
            Self::Float(v) => (Type::Float, ValueRepr { float: v }),
            Self::Int(v) => (Type::Int, ValueRepr { int: v }),
            Self::LightUserdata(v) => (Type::LightUserdata, ValueRepr { light_userdata: v }),
            Self::FullUserdata(v) => (Type::FullUserdata, ValueRepr { full_userdata: v }),
            Self::Function(v) => (Type::Function, ValueRepr { function: v }),
            Self::String(v) => (Type::String, ValueRepr { string: v }),
            Self::Table(v) => (Type::Table, ValueRepr { table: v }),
            Self::Thread(v) => (Type::Thread, ValueRepr { thread: v }),
        }
    }
}

struct FullUserdata {
    metatable: Option<NonNull<Table>>,
}

struct Table {
    metatable: Option<NonNull<Table>>,
    sequence_storage: *mut u8,
    sequence_capacity: usize,
    sequence_len: usize,
    map_storage: *mut u8,
    map_capacity: usize,
}

struct FunctionFamily {
    captures: NonNull<[Type]>,
}

struct Function {
    family: NonNull<FunctionFamily>,
}

struct String {
    len: usize,
    hash: usize,
}

struct Thread {
    context: NonNull<Context>,
}

struct Context {
    data_allocator: DataAllocator,
    code_allocator: CodeAllocator,
    main_thread: NonNull<Thread>,
}

struct DataAllocator {}

struct CodeAllocator {}

fn main() {
    println!("Hello, world!");
}
