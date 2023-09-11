#[macro_export]
macro_rules! entity_ref_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        cranelift_entity::entity_impl!($name);
    };
}
