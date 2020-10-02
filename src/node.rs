use crate::tree::{PoolTree, Index};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::iter::Iter;

pub struct NodeTop<'a, T> {
    inner: NodeMut<'a, T>,
}

impl<'a, T> NodeTop<'a, T> {
    pub unsafe fn create(index: Index, buffer: *mut PoolTree<T>) -> Self {
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
        println!("Add Child!");
        unsafe {
            //Insert Element
            let index = (&mut*self.buffer).alloc_for(value, self.index);

            //Reference Element
            let children = &mut (&mut*self.buffer).get_raw_mut(self.index).first_last_child;
            if let Some((_, ref mut last)) = children {

                println!("Follow Child!");

                let previous = (&mut*self.buffer).get_raw_mut(last.get());
                previous.next_sibling = Some(index);

                let this_child = (&mut*self.buffer).get_raw_mut(index.get());
                this_child.prev_sibling = Some(*last);

                *last = index;

            } else {
                println!("First Child!");
                *children = Some((index, index));
            }

            NodeMut::create(index.get(), self.buffer)
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
    index: Index
}

impl<'a, T> NodeMut<'a, T> {
    pub unsafe fn create(index: Index, buffer: *mut PoolTree<T>) -> Self {
        NodeMut {
            index,
            buffer,
            _p: PhantomData
        }
    }
}

impl<'a, T> Deref for NodeMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            & *(& *self.buffer).get_raw(self.index).value.as_ptr()
        }
    }
}

impl<'a, T> DerefMut for NodeMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(&mut *self.buffer).get_raw_mut(self.index).value.as_mut_ptr()
        }
    }
}

pub struct Node<'a, T> {
    buffer: &'a PoolTree<T>,
    index: Index,
}

impl<'a, T> Node<'a, T> {
    pub unsafe fn create(index: Index, buffer: &'a PoolTree<T>) -> Self {
        Node {
            index,
            buffer,
        }
    }

    pub fn childs(&self) -> Iter<T> {
        Iter{
            buffer: self.buffer,
            range: unsafe { self.buffer.get_raw(self.index) }.first_last_child,
        }
    }
}

impl<'a, T> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            & *self.buffer.get_raw(self.index).value.as_ptr()
        }
    }
}