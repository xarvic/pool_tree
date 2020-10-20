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

impl<'a, T: 'static> TreeRef for Ref<'a, T> {
    type Type = T;
    type Children<'b> = Ref<'b, T>;

    unsafe fn create(buffer: *const Tree<T>, index: u32) -> Self {
        Self::create(index, &*buffer)
    }

    fn index(&self) -> u32 {
        self.index
    }

    fn children<'c>(&'c self) -> ChildIter<'c, Self::Type, Self::Children<'c>> {
        let buffer = self.buffer;
        unsafe {
            ChildIter::new(buffer, self.raw().childs())
        }
    }

    fn get_child<'b>(&'b self, index: u32) -> Self::Children<'b> {
        unsafe {
            let index = *self.raw().childs().get(index as usize).expect("index out of bounds!");
            Ref::create(index.get(), self.buffer)
        }
    }

    fn children_count(&self) -> u32 {
        unsafe { self.raw() }.childs().len() as u32
    }

    fn get_ref(&self) -> Ref<Self::Type> {
        unsafe { Self::create(self.index, self.buffer) }
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
    type Type: 'static;
    type Children<'a>: TreeRef<Type=Self::Type>;
    unsafe fn create(buffer: *const Tree<Self::Type>, index: u32) -> Self;

    fn index(&self) -> u32;

    fn children<'b>(&'b self) -> ChildIter<'b, Self::Type, Self::Children<'b>>;

    fn get_child<'b>(&'b self, index: u32) -> Self::Children<'b>;

    fn children_count(&self) -> u32;

    fn get_ref<'b>(&'b self) -> Ref<'b, Self::Type>;
}

impl<'a, T: Display + 'static> Display for Ref<'a, T> {
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