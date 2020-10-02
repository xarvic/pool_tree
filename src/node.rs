use crate::tree::{PoolTree, Element};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct NodeTop<'a, T> {
    inner: NodeMut<'a, T>,
}

impl<'a, T> NodeTop<'a, T> {
    pub unsafe fn create(index: u32, buffer: *mut PoolTree<T>) -> Self {
        NodeTop {
            inner: NodeMut {
                index,
                buffer,
                _p: PhantomData
            }
        }
    }
    pub fn inner(self) -> NodeMut<'a, T> {
        self.inner
    }

    pub fn add_child(&mut self, value: T) -> NodeMut<T> {
        unsafe {
            let index = (*self.buffer).alloc_for(value, self.index);
            self.raw_mut().childs_mut().push(index);

            NodeMut::create(index.get(), self.buffer)
        }
    }

    pub fn remove_child(&mut self, index: u32) -> T {
        unsafe {
            let childs = self.raw_mut().childs_mut();
            if childs.len() > index as usize {

                let id = childs.remove(index as usize);
                (&mut*self.buffer).free(id)

            } else {
                panic!("Index out of Bounds!")
            }
            //TODO: remove complete Tree
        }
    }
}

impl<'a, T> Deref for NodeTop<'a, T> {
    type Target = NodeMut<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for NodeTop<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct NodeMut<'a, T> {
    _p: PhantomData<&'a mut PoolTree<T>>,
    buffer: *mut PoolTree<T>,
    index: u32
}

impl<'a, T> NodeMut<'a, T> {
    pub unsafe fn create(index: u32, buffer: *mut PoolTree<T>) -> Self {
        NodeMut {
            index,
            buffer,
            _p: PhantomData
        }
    }

    pub fn children(&mut self) -> impl Iterator<Item=NodeMut<T>> {
        let buffer = self.buffer;
        unsafe {
            self.raw().childs()
                .iter().map(move|index|NodeMut::create(index.get(), buffer))
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

impl<'a, T> Deref for NodeMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.raw().get_value()
        }
    }
}

impl<'a, T> DerefMut for NodeMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.raw_mut().get_value_mut()
        }
    }
}

pub struct Node<'a, T> {
    buffer: &'a PoolTree<T>,
    index: u32,
}

impl<'a, T> Node<'a, T> {
    pub unsafe fn create(index: u32, buffer: &'a PoolTree<T>) -> Self {
        Node {
            index,
            buffer,
        }
    }

    pub fn children(&self) -> impl Iterator<Item = Node<T>> {
        unsafe {
            self.buffer.get_raw(self.index).childs()
                .iter().map(move|index|Node::create(index.get(), self.buffer))
        }
    }
}

impl<'a, T> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.buffer.get_raw(self.index).get_value()
        }
    }
}