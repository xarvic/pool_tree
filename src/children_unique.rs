use crate::iter::ChildIter;
use crate::reference::Ref;
use crate::ref_mut::RefMut;
use crate::child_unique::ChildUniq;

pub struct ChildrenUnique<'a, T> {
    inner: ChildUniq<'a, T>,
}

impl<'a, T: 'static> ChildrenUnique<'a, T> {
    pub fn create(value: ChildUniq<'a, T>) -> Self {
        ChildrenUnique {
            inner: value,
        }
    }
    pub fn id(&mut self) -> ChildrenUnique<T> {
        ChildrenUnique{
            inner: self.inner.id()
        }
    }
    pub fn get_child_unique(&mut self, index: u32) -> ChildUniq<T> {
        self.inner.get_child_unique(index)
    }
    pub fn iter(&self) -> ChildIter<T, Ref<T>> {
        unsafe {
            ChildIter::new(self.inner.buffer(), self.inner.raw().childs())
        }
    }
    pub fn iter_mut(&mut self) -> ChildIter<T, RefMut<T>> {
        unsafe {
            ChildIter::new(self.inner.buffer(), self.inner.raw().childs())
        }
    }
    pub fn add_child(&mut self, value: T) -> ChildUniq<T> {
        self.inner.add_child(value)
    }
}