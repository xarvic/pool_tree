use std::ops::{Receiver, DerefMut, Deref};
use crate::tree::{Element, Tree};
use std::marker::PhantomData;
use crate::reference::{TreeRef, Ref};
use crate::iter::ChildIter;

pub struct RefMut<'a, T> {
    _p: PhantomData<&'a mut Tree<T>>,
    pub(crate) buffer: *mut Tree<T>,
    pub(crate) index: u32
}

impl<'a, T> RefMut<'a, T> {
    pub unsafe fn create(index: u32, buffer: *mut Tree<T>) -> Self {
        RefMut {
            index,
            buffer,
            _p: PhantomData
        }
    }
    pub(crate) unsafe fn raw(&self) -> &Element<T> {
        (& *self.buffer).get_raw(self.index)
    }
    pub(crate) unsafe fn raw_mut(&mut self) -> &mut Element<T> {
        (&mut *self.buffer).get_raw_mut(self.index)
    }
    pub(crate) unsafe fn raw_index(&mut self, index: u32) -> &Element<T> {
        (& *self.buffer).get_raw(index)
    }
    pub(crate) unsafe fn raw_index_mut(&mut self, index: u32) -> &mut Element<T> {
        (&mut *self.buffer).get_raw_mut(index)
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.raw().get_value()
        }
    }
}

impl<'a, T> Receiver for RefMut<'a, T>{}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.raw_mut().get_value_mut()
        }
    }
}

impl<'a, T: 'static> TreeRef for RefMut<'a, T> {
    type Type = T;
    type Children<'b> = Ref<'b, T>;

    unsafe fn create(buffer: *const Tree<Self::Type>, index: u32) -> Self {
        Self::create(index, buffer as *mut Tree<Self::Type>)
    }

    fn index(&self) -> u32 {
        self.index
    }

    fn children<'b>(&'b self) -> ChildIter<'b, Self::Type, Self::Children<'b>> {
        let buffer = self.buffer;
        unsafe {
            ChildIter::new(buffer, self.raw().childs())
        }
    }

    fn children_count(&self) -> u32 {
        unsafe { self.raw() }.childs().len() as u32
    }

    fn get_ref(&self) -> Ref<Self::Type> {
        unsafe {Ref::create(self.index, &*self.buffer)}
    }
}

impl<'a, T: 'static> TreeRefMut for RefMut<'a, T> {
    fn children_mut(&mut self) -> ChildIter<Self::Type, RefMut<Self::Type>> {
        let buffer = self.buffer;
        unsafe {
            ChildIter::new(buffer, self.raw().childs())
        }
    }

    fn both(&mut self) -> (&mut Self::Type, ChildIter<Self::Type, RefMut<Self::Type>>) {
        unsafe {
            let this = self as *mut Self;
            let value = (&mut *this).raw_mut().get_value_mut();

            (value, self.children_mut())
        }
    }
}

pub trait TreeRefMut: TreeRef {
    fn children_mut(&mut self) -> ChildIter<Self::Type, RefMut<Self::Type>>;
    fn both(&mut self) -> (&mut Self::Type, ChildIter<Self::Type, RefMut<Self::Type>>);
}