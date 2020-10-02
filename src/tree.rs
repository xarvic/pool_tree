use crate::node::{NodeTop, Node};
use std::num::NonZeroU32;
use std::mem::replace;
use std::fmt::{Debug, Formatter};
use smallvec::SmallVec;
use std::hint::unreachable_unchecked;

pub struct Element<T> {
    pub value: Option<T>,
    pub parent_next_free: Option<NonZeroU32>,
    pub childs: SmallVec<[NonZeroU32; 5]>,

}

impl<T> Element<T> {
    pub fn new(value: T, parent: u32) -> Self {
        Element {
            value: Some(value),
            parent_next_free: NonZeroU32::new(parent),
            childs: SmallVec::new(),
        }
    }
    pub unsafe fn unused(next: Option<NonZeroU32>) -> Self {
        Element{
            value: None,
            parent_next_free: next,
            childs: SmallVec::new(),
        }
    }

    pub unsafe fn get_value(&self) -> &T {
        self.value.as_ref().unwrap_or_else(||unreachable_unchecked())
    }

    pub unsafe fn get_value_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap_or_else(||unreachable_unchecked())
    }
    pub fn is_used(&self) -> bool {
        self.value.is_some()
    }
    pub fn set_used(&mut self, value: T, parent: u32) -> Option<NonZeroU32> {
        self.value.replace(value);
        replace(&mut self.parent_next_free, NonZeroU32::new(parent))
    }
    pub fn set_unused(&mut self, next_free: Option<NonZeroU32>) -> T {
        self.parent_next_free = next_free;
        self.value.take().unwrap_or_else(||panic!())
    }
    pub fn set_parent(&mut self, index: u32) {
        self.parent_next_free = NonZeroU32::new(index);
    }
    pub fn parent(&self) -> u32 {
        self.parent_next_free.map_or(0, |n|n.get())
    }
    pub fn set_next_free(&mut self, next: Option<NonZeroU32>) {
        self.parent_next_free = next;
    }
    pub fn next_free(&self) -> Option<NonZeroU32> {
        self.parent_next_free.clone()
    }
    pub fn childs(&self) -> &[NonZeroU32] {
        &*self.childs
    }
    pub fn childs_mut(&mut self) -> &mut SmallVec<[NonZeroU32; 5]> {
        &mut self.childs
    }
}

impl<T: Debug> Debug for Element<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_used() {
            f.debug_struct("Element")
                .field("value", &self.value)
                .field("parent", &self.parent())
                .field("childs", &self.childs())
                .finish()
        } else {
            f.debug_struct("Element (unused)")
                .field("value", &self.value)
                .field("next_free", &self.next_free())
                .finish()
        }
    }
}

pub struct PoolTree<T> {
    buffer: Vec<Element<T>>,
    next_free: Option<NonZeroU32>,
}

impl<T> PoolTree<T> {
    pub fn new(root: T) -> Self {
        PoolTree{
            buffer: vec![Element::new(root, 0)],
            next_free: None,
        }
    }

    pub(crate) unsafe fn alloc_for(&mut self, value: T, parent: u32) -> NonZeroU32 {
        if let Some(index) = self.next_free {
            let element = self.get_raw_mut(index.get());
            self.next_free = element.set_used(value, parent);
            index
        } else {
            self.buffer.push(Element::new(value, parent));

            //Buffer is never empty, therefore is the new last Index greater than 0
            NonZeroU32::new_unchecked((self.buffer.len() - 1) as u32)
        }
    }
    pub(crate) unsafe fn free(&mut self, index: NonZeroU32) -> T {
        let previous_free = replace(&mut self.next_free, Some(index));
        let element = self.get_raw_mut(index.get());
        element.set_unused(previous_free)
    }

    pub(crate) unsafe fn get_raw(&self, index: u32) -> &Element<T> {
        self.buffer.get_unchecked(index as usize)
    }

    pub(crate) unsafe fn get_raw_mut(&mut self, index: u32) -> &mut Element<T> {
        self.buffer.get_unchecked_mut(index as usize)
    }

    pub unsafe fn get_unchecked(&self, index: u32) -> Node<T> {
        Node::create(index, self as _)
    }

    pub fn get_index(&self, index: u32) -> Option<Node<T>> {
        if self.buffer.get(index as usize).map_or(false, |element|element.is_used()) {
            Some(unsafe {self.get_unchecked(index)})
        } else {
            None
        }
    }
    pub fn top(&self) -> Node<T> {
        unsafe {self.get_unchecked(0) }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: u32) -> NodeTop<T> {
        NodeTop::create(index, self as _)
    }
    pub fn get_index_mut(&mut self, index: u32) -> Option<NodeTop<T>> {
        if self.buffer.get(index as usize).map_or(false, |element|element.is_used()) {
            Some(unsafe {self.get_unchecked_mut(index)})
        } else {
            None
        }
    }
    pub fn mut_top(&mut self) -> NodeTop<T> {
        unsafe {self.get_unchecked_mut(0) }
    }
}

impl<T: Debug> Debug for PoolTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}