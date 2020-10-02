use crate::node::{NodeTop, Node};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, replace};
use std::fmt::{Debug, Formatter};

pub type Index = usize;
pub type NZIndex = NonZeroUsize;

pub struct Element<T> {
    pub(crate) value: MaybeUninit<T>,
    pub(crate) used: bool,
    pub(crate) parent: Index,
    pub(crate) next_sibling: Option<NonZeroUsize>,
    pub(crate) prev_sibling: Option<NonZeroUsize>,
    pub(crate) first_last_child: Option<(NonZeroUsize, NonZeroUsize)>,
}

impl<T> Element<T> {
    pub fn new(value: T, parent: Index) -> Self {
        Element {
            value: MaybeUninit::new(value),
            used: true,
            parent,
            next_sibling: None,
            prev_sibling: None,
            first_last_child: None,
        }
    }
    pub unsafe fn unused(next: Option<NonZeroUsize>) -> Self {
        Element{
            value: MaybeUninit::uninit(),
            used: false,
            parent: 0,
            next_sibling: next,
            prev_sibling: None,
            first_last_child: None,
        }
    }

    pub unsafe fn get_value(&self) -> &T {
        &*self.value.as_ptr()
    }

    pub unsafe fn get_value_mut(&mut self) -> &mut T {
        &mut *self.value.as_mut_ptr()
    }
}

impl<T: Debug> Debug for Element<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("value", &self.value)
            .field("used", &self.used)
            .field("parent", &self.parent)
            .field("next_sibling", &self.next_sibling)
            .field("prev_sibling", &self.prev_sibling)
            .field("first_last_child", &self.first_last_child)
            .finish()
    }
}

pub struct PoolTree<T> {
    buffer: Vec<Element<T>>,
    next_free: Option<NZIndex>,
}

impl<T> PoolTree<T> {
    pub fn new(root: T) -> Self {
        PoolTree{
            buffer: vec![Element::new(root, 0)],
            next_free: None,
        }
    }

    pub(crate) unsafe fn alloc_for(&mut self, value: T, parent: Index) -> NZIndex {
        if let Some(index) = self.next_free {
            let element = self.get_raw_mut(index.get());
            *element = Element::new(value, parent);
            self.next_free = element.next_sibling;
            index
        } else {
            self.buffer.push(Element::new(value, parent));
            NonZeroUsize::new_unchecked(self.buffer.len() - 1)
        }
    }
    pub(crate) unsafe fn free(&mut self, index: NZIndex) {
        let previous_free = replace(&mut self.next_free, Some(index));
        let element = self.get_raw_mut(index.get());
        element.used = false;
        element.next_sibling = previous_free;
    }

    pub(crate) unsafe fn get_raw(&self, index: Index) -> &Element<T> {
        self.buffer.get_unchecked(index)
    }

    pub(crate) unsafe fn get_raw_mut(&mut self, index: Index) -> &mut Element<T> {
        self.buffer.get_unchecked_mut(index)
    }

    pub unsafe fn get_unchecked(&self, index: Index) -> Node<T> {
        Node::create(index, self as _)
    }

    pub fn get_index(&self, index: Index) -> Option<Node<T>> {
        if self.buffer.get(index).map_or(false, |element|element.used) {
            Some(unsafe {self.get_unchecked(index)})
        } else {
            None
        }
    }
    pub fn top(&self) -> Node<T> {
        unsafe {self.get_unchecked(0) }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: Index) -> NodeTop<T> {
        NodeTop::create(index, self as _)
    }
    pub fn get_index_mut(&mut self, index: Index) -> Option<NodeTop<T>> {
        if self.buffer.get(index).map_or(false, |element|element.used) {
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