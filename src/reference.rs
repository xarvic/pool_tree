use std::ops::{Receiver, Deref};
use smallvec::alloc::fmt::{Display, Formatter};
use crate::tree::{Tree, Element};
use crate::iter::ChildIter;

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
    unsafe fn raw(&self) -> &Element<T> {
        &self.buffer.get_raw(self.index)
    }
}

impl<'a, T> Receiver for Ref<'a, T>{}

impl<'a, T> TreeRef for Ref<'a, T> {
    type Type = T;
    type Children = Self;

    unsafe fn create(buffer: *const Tree<T>, index: u32) -> Self {
        Self::create(index, &*buffer)
    }

    fn index(&self) -> u32 {
        self.index
    }

    fn children(&self) -> ChildIter<Self::Type, Self::Children> {
        let buffer = self.buffer;
        unsafe {
            ChildIter::new(buffer, self.raw().childs())
        }
    }

    fn children_count(&self) -> u32 {
        unsafe { self.raw() }.childs().len() as u32
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.raw().get_value()
        }
    }
}

pub trait TreeRef {
    type Type;
    type Children: TreeRef<Type=Self::Type>;
    unsafe fn create(buffer: *const Tree<Self::Type>, index: u32) -> Self;

    fn index(&self) -> u32;
    fn children(&self) -> ChildIter<Self::Type, Self::Children>;
    fn children_count(&self) -> u32;
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

//TODO: impl Eq and Debug, Display, ToOwned for TreeRef