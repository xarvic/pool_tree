use crate::tree::Tree;
use std::num::NonZeroU32;
use crate::iter::ChildIter;
use crate::reference::Ref;
use crate::ref_mut::RefMut;
use crate::child_unique::ChildUniq;

pub struct ChildrenUnique<'a, T> {
    buffer: *mut Tree<T>,
    child_indices: &'a [NonZeroU32],
}

impl<'a, T: 'static> ChildrenUnique<'a, T> {
    pub unsafe fn create(buffer: *mut Tree<T>, child_indices: &'a [NonZeroU32]) -> Self {
        ChildrenUnique {
            buffer,
            child_indices,
        }
    }
    pub fn id(&mut self) -> ChildrenUnique<'a, T> {
        unsafe {
            ChildrenUnique::create(self.buffer, self.child_indices)
        }
    }
    pub fn get_child_uniqe(&mut self, index: u32) -> ChildUniq<'a, T> {
        let child_index = self.child_indices.get(index as usize).expect("Index out of bounds!");
        unsafe {ChildUniq::create(self.buffer, child_index.get())}
    }
    pub fn iter(&self) -> ChildIter<T, Ref<T>> {
        unsafe {
            ChildIter::new(self.buffer, self.child_indices)
        }
    }
    pub fn iter_mut(&mut self) -> ChildIter<T, RefMut<T>> {
        unsafe {
            ChildIter::new(self.buffer, self.child_indices)
        }
    }
}