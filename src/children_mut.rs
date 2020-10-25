use crate::tree::Tree;
use std::num::NonZeroU32;
use crate::iter::ChildIter;
use crate::reference::Ref;
use crate::ref_mut::RefMut;

pub struct ChildrenMut<'a, T> {
    buffer: *mut Tree<T>,
    child_indices: &'a [NonZeroU32],
}

impl<'a, T: 'static> ChildrenMut<'a, T> {
    pub unsafe fn create(buffer: *mut Tree<T>, child_indices: &'a [NonZeroU32]) -> Self {
        ChildrenMut {
            buffer,
            child_indices,
        }
    }
    pub fn id(&mut self) -> ChildrenMut<'a, T> {
        unsafe {
            ChildrenMut::create(self.buffer, self.child_indices)
        }
    }
    pub fn get_child_mut(&mut self, index: u32) -> RefMut<'a, T> {
        let child_index = self.child_indices.get(index as usize).expect("Index out of bounds!");
        unsafe {RefMut::create(child_index.get(), self.buffer)}
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