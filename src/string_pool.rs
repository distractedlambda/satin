use {
    crate::entity_ref_type,
    ahash::AHashMap,
    bumpalo::Bump,
    cranelift_entity::PrimaryMap,
    std::{cell::UnsafeCell, ops::Index},
};

entity_ref_type!(StringRef);

pub struct StringPool {
    storage: Bump,
    strings: UnsafeCell<PrimaryMap<StringRef, *const [u8]>>,
    dedup_table: UnsafeCell<AHashMap<&'static [u8], StringRef>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            storage: Bump::new(),
            strings: UnsafeCell::new(PrimaryMap::new()),
            dedup_table: UnsafeCell::new(AHashMap::new()),
        }
    }

    pub fn intern(&self, string: &[u8]) -> StringRef {
        unsafe {
            if let Some(&existing) = (*self.dedup_table.get().cast_const()).get(string) {
                existing
            } else {
                let string = self.storage.alloc_slice_copy(string) as *const [u8];
                let new = (*self.strings.get()).push(string);
                (*self.dedup_table.get()).insert(&*string, new);
                new
            }
        }
    }

    pub fn clear(&mut self) {
        self.dedup_table.get_mut().clear();
        self.strings.get_mut().clear();
        self.storage.reset();
    }
}

impl Index<StringRef> for StringPool {
    type Output = [u8];

    fn index(&self, index: StringRef) -> &Self::Output {
        unsafe { &*(*self.strings.get().cast_const())[index] }
    }
}
