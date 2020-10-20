use std::ops::{Receiver, Deref};
use smallvec::alloc::fmt::{Display, Formatter};
use crate::tree::Tree;

pub struct Ref<'a, T> {
    buffer: &'a Tree<T>,
    index: u32,
}

impl<'a, T> Ref<'a, T> {
    pub unsafe fn create(index: u32, buffer: &'a Tree<T>) -> Self {
        Ref {
            index,
            buffer,
        }
    }

    pub fn children(&self) -> impl Iterator<Item = Ref<T>> {
        unsafe {
            self.buffer.get_raw(self.index).childs()
                .iter().map(move|index| Ref::create(index.get(), self.buffer))
        }
    }
    pub fn index(&self) -> u32 {
        self.index
    }


}

impl<'a, T> Receiver for Ref<'a, T>{}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.buffer.get_raw(self.index).get_value()
        }
    }
}

impl<'a, T: Display> Display for Ref<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)?;
        let mut iter = self.children();
        if let Some(value) = iter.next() {
            f.write_str("(")?;
            value.fmt(f)?;

            for next in iter {
                f.write_str(", ")?;
                next.fmt(f)?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}