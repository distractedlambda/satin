use std::cell::UnsafeCell;

#[repr(transparent)]
pub struct VecCell<T>(UnsafeCell<Vec<T>>);

impl<T> VecCell<T> {
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity).into()
    }

    pub fn get_mut(&mut self) -> &mut Vec<T> {
        self.0.get_mut()
    }

    unsafe fn expose_ref(&self) -> &Vec<T> {
        &*self.0.get().cast_const()
    }

    unsafe fn expose_mut(&self) -> &mut Vec<T> {
        &mut *self.0.get()
    }

    pub fn capacity(&self) -> usize {
        unsafe { self.expose_ref().capacity() }
    }

    pub fn reserve(&self, additional: usize) {
        unsafe { self.expose_mut().reserve(additional) }
    }

    pub fn reserve_exact(&self, additional: usize) {
        unsafe { self.expose_mut().reserve_exact(additional) }
    }

    pub fn shrink_to_fit(&self) {
        unsafe { self.expose_mut().shrink_to_fit() }
    }

    pub fn swap_remove(&self, index: usize) -> T {
        unsafe { self.expose_mut().swap_remove(index) }
    }

    pub fn insert(&self, index: usize, element: T) {
        unsafe { self.expose_mut().insert(index, element) }
    }

    pub fn remove(&self, index: usize) -> T {
        unsafe { self.expose_mut().remove(index) }
    }

    pub fn push(&self, element: T) {
        unsafe { self.expose_mut().push(element) }
    }

    pub fn pop(&self) -> Option<T> {
        unsafe { self.expose_mut().pop() }
    }

    pub fn clear(&self) {
        unsafe { self.expose_mut().clear() }
    }

    pub fn len(&self) -> usize {
        unsafe { self.expose_ref().len() }
    }
}

impl<T> Default for VecCell<T> {
    fn default() -> Self {
        Vec::new().into()
    }
}

impl<T> From<Vec<T>> for VecCell<T> {
    fn from(value: Vec<T>) -> Self {
        Self(UnsafeCell::new(value))
    }
}

impl<T> From<VecCell<T>> for Vec<T> {
    fn from(value: VecCell<T>) -> Self {
        value.0.into_inner()
    }
}
