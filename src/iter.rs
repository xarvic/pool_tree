use smallvec::alloc::slice::Iter;
use crate::tree::Tree;
use std::marker::PhantomData;
use crate::reference::TreeRef;
use std::num::NonZeroU32;

pub struct ChildIter<'a, T, R: 'a + TreeRef<T>> {
    buffer: *mut Tree<T>,
    children_indices: Iter<'a, NonZeroU32>,
    gen: PhantomData<fn()->R>,
}

impl<'a, T, R: 'a + TreeRef<T>> ChildIter<'a, T, R> {
    pub unsafe fn new(buffer: *mut Tree<T>, indices: &'a [NonZeroU32]) -> Self {
        ChildIter{
            buffer,
            children_indices: indices.into_iter(),
            gen: PhantomData,
        }
    }
}

impl<'a, T, R: 'a + TreeRef<T>> Iterator for ChildIter<'a, T, R> {
    type Item = R;

    fn next(&mut self) -> Option<R> {
        let buffer = self.buffer;
        self.children_indices.next().map(|index|unsafe {
            R::create(buffer, index.get())
        })
    }
}