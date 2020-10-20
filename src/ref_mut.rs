use std::ops::{Receiver, DerefMut, Deref};
use crate::tree::{Element, Tree};
use std::marker::PhantomData;

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
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn children(&mut self) -> impl Iterator<Item=RefMut<T>> {
        let buffer = self.buffer;
        unsafe {
            self.raw().childs()
                .iter().map(move|index| RefMut::create(index.get(), buffer))
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