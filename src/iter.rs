use crate::tree::{PoolTree, NZIndex};
use crate::node::Node;
use std::hint::unreachable_unchecked;

pub struct Iter<'a, T> {
    pub(crate) buffer: &'a PoolTree<T>,
    pub(crate) range: Option<(NZIndex, NZIndex)>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, end)) = self.range {
            unsafe {
                self.range = if start == end {
                    None
                } else {
                    Some((self.buffer.get_raw(start.get()).next_sibling.unwrap_or_else(||unreachable_unchecked()), end))
                };
            }
            Some(unsafe { Node::create(start.get(), self.buffer) })
        } else {
            None
        }
    }
}

