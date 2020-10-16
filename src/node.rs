use crate::tree::{Tree, Element};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Receiver};

pub struct RefUniq<'a, T> {
    inner: RefMut<'a, T>,
}

impl<'a, T> RefUniq<'a, T> {
    pub unsafe fn create(index: u32, buffer: *mut Tree<T>) -> Self {
        RefUniq {
            inner: RefMut {
                index,
                buffer,
                _p: PhantomData
            }
        }
    }
    pub fn inner(self) -> RefMut<'a, T> {
        self.inner
    }

    pub fn add_child(&mut self, value: T) -> RefMut<T> {
        unsafe {
            let index = (*self.buffer).alloc_for(value, self.index);
            self.raw_mut().children_mut().push(index);

            RefMut::create(index.get(), self.buffer)
        }
    }

    pub fn remove_child(&mut self, index: u32) -> T {
        unsafe {
            let childs = self.raw_mut().children_mut();
            if childs.len() > index as usize {

                let id = childs.remove(index as usize);
                (&mut*self.buffer).free(id)

            } else {
                panic!("Index out of Bounds!")
            }
            //TODO: remove complete Tree
        }
    }

    pub fn into_child(self, index: u32) -> Result<Self, Self> {
        unsafe {
            if let Some(index) = self.raw().childs().get(index as usize) {
                Ok(RefUniq::create(index.get(), self.buffer))
            } else {
                Err(self)
            }
        }

    }
    pub fn into_parent(self) -> Result<Self, Self> {
        unsafe {
            if self.index != 0 {
                Ok(RefUniq::create(self.raw().parent(), self.buffer))
            } else {
                Err(self)
            }
        }
    }

}

impl<'a, T> Receiver for RefUniq<'a, T>{}

impl<'a, T> Deref for RefUniq<'a, T> {
    type Target = RefMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for RefUniq<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct RefMut<'a, T> {
    _p: PhantomData<&'a mut Tree<T>>,
    buffer: *mut Tree<T>,
    index: u32
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
    unsafe fn raw(&self) -> &Element<T> {
        (& *self.buffer).get_raw(self.index)
    }
    unsafe fn raw_mut(&mut self) -> &mut Element<T> {
        (&mut *self.buffer).get_raw_mut(self.index)
    }
    unsafe fn raw_index(&mut self, index: u32) -> &Element<T> {
        (& *self.buffer).get_raw(index)
    }
    unsafe fn raw_index_mut(&mut self, index: u32) -> &mut Element<T> {
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

impl<'a, T> Receiver for RefUniq<'a, T>{}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.raw_mut().get_value_mut()
        }
    }
}

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

impl<'a, T> Receiver for RefUniq<'a, T>{}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.buffer.get_raw(self.index).get_value()
        }
    }
}