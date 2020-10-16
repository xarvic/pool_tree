use crate::node::{RefUniq, Ref};
use std::num::NonZeroU32;
use std::mem::replace;
use std::fmt::{Debug, Formatter};
use smallvec::SmallVec;
use std::hint::unreachable_unchecked;

/// Element stores the value of a Node as well as the indices of its parent and its children.
/// The value field uses an Option<T> to avoid an extra field used. parent_next_is the index of the
/// parent if value is Some and the next unused value if the value is None.
pub struct Element<T> {
    pub value: Option<T>,
    pub parent_next_free: Option<NonZeroU32>,
    pub childs: SmallVec<[NonZeroU32; 5]>,

}

//TODO: decide whether the methods should panic or trigger undefined behaviour
impl<T> Element<T> {
    /// Creates a new used Element with no Children and the given value and parent
    pub fn new(value: T, parent: u32) -> Self {
        Element {
            value: Some(value),
            parent_next_free: NonZeroU32::new(parent),
            childs: SmallVec::new(),
        }
    }
    /// Creates a new unused Element with the given next_free index, value is None
    pub unsafe fn unused(next: Option<NonZeroU32>) -> Self {
        Element{
            value: None,
            parent_next_free: next,
            childs: SmallVec::new(),
        }
    }

    /// returns a reference to the value.
    /// The safe counterpart to this method is [`get_value_checked`]
    ///
    /// #Safety
    /// This method assumes that the value is present if not the behaviour is undefined. Ref uses
    /// this method since the Ref itself only gets constructed with indices to valid used Elements
    #[inline]
    pub unsafe fn get_value(&self) -> &T {
        self.value.as_ref().unwrap_or_else(||unreachable_unchecked())
    }

    /// returns a reference to the value.
    /// The safe counterpart to this method is [`get_value_checked_mut`]
    ///
    /// #Safety
    /// This method assumes that the value is present if not the behaviour is undefined. Ref uses
    /// this method since the Ref itself only gets constructed with indices to valid used Elements
    #[inline]
    pub unsafe fn get_value_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap_or_else(||unreachable_unchecked())
    }

    /// returns a reference to the value if it is present
    #[inline]
    pub fn get_value_checked(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// returns a mutable reference to the value if it is present
    #[inline]
    pub fn get_value_checked_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    /// returns true if the value is present and therefore part of the Tree
    #[inline]
    pub fn is_used(&self) -> bool {
        self.value.is_some()
    }

    /// equivalent to *self = Element::new(value, parent)
    ///
    /// assumes, that the value was previosly unused and returns the next unused value
    #[inline]
    pub fn set_used(&mut self, value: T, parent: u32) -> Option<NonZeroU32> {
        self.value.replace(value);
        replace(&mut self.parent_next_free, NonZeroU32::new(parent))
    }

    /// equivalent to ```*self = Element::unused(next_free)``` and returns the value of the Element
    ///
    /// #Panics
    /// Panics if the Element was already unused
    #[inline]
    pub fn set_unused(&mut self, next_free: Option<NonZeroU32>) -> T {
        self.parent_next_free = next_free;
        self.value.take().unwrap_or_else(||panic!("freed an unused Element!"))
    }

    /// changed the parent of the Element
    ///
    /// #Panics
    /// Panics if the Element is unused
    #[inline]
    pub fn set_parent(&mut self, index: u32) {
        if self.is_used() {
            self.parent_next_free = NonZeroU32::new(index);
        } else {
            panic!("Changed Parent of unused Element!");
        }
    }

    /// returns the parent of the Node
    /// if the Element is unused this method returns an arbitrary number!
    #[inline]
    pub fn parent(&self) -> u32 {
        self.parent_next_free.map_or(0, |n|n.get())
    }

    /// Sets the next free value
    ///
    /// #Panics
    /// Panics if the value is used
    #[inline]
    pub fn set_next_free(&mut self, next: Option<NonZeroU32>) {

        if !self.is_used() {
            self.parent_next_free = next;
        } else {
            panic!("set next free of used Element!");
        }
    }

    /// returns the next free index of the free Elements Queue
    /// if the Element is unused this method returns an arbitrary number!
    #[inline]
    pub fn next_free(&self) -> Option<NonZeroU32> {
        self.parent_next_free.clone()
    }

    ///Returns the indices of all children of this Node
    #[inline]
    pub fn childs(&self) -> &[NonZeroU32] {
        &*self.childs
    }

    ///
    //TODO: dont leak internal details: impl SomeCollectionTrait<NonZeroU32>
    #[inline]
    pub fn children_mut(&mut self) -> &mut SmallVec<[NonZeroU32; 5]> {
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

/// Tree is a never empty tree, with its nodes stored in a Vec which serves as a pool allocator.
/// The root node is always at index 0 which allows for some optimisations:
///  - the indices of child nodes are represented by NonZeroU32
///  - the top and top_mut methods return Ref and RefUniq instead of Option<Ref> and Option<RefUniq>
///    since the top value is always present
///
pub struct Tree<T> {
    buffer: Vec<Element<T>>,
    next_free: Option<NonZeroU32>,
}

impl<T> Tree<T> {

    ///creates a new Tree with the given root Node
    #[inline]
    pub fn new(root: T) -> Self {
        Tree {
            buffer: vec![Element::new(root, 0)],
            next_free: None,
        }
    }

    /// allocates an Element for a Node, with the given value and parent
    ///
    /// #Safety
    /// This method may reallocate the element-buffer.
    /// The caller has to ensure, that no references into the buffer exist, when calling this
    /// method
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

    /// frees the Element at the given index
    ///
    /// #Panics
    /// This method panics if the given index is unused, or outside the uffers range
    pub(crate) unsafe fn free(&mut self, index: NonZeroU32) -> T {
        assert!(index < self.buffer.len());

        let previous_free = replace(&mut self.next_free, Some(index));
        let element = self.get_raw_mut(index.get());
        element.set_unused(previous_free)
    }

    /// returns the Element at index
    ///
    /// #Safety
    /// if the index is outside of the buffers bounds the behaviour is undefined
    pub unsafe fn get_raw(&self, index: u32) -> &Element<T> {
        self.buffer.get_unchecked(index as usize)
    }

    /// returns the Element at index
    ///
    /// #Safety
    /// if the index is outside of the buffers bounds the behaviour is undefined
    pub unsafe fn get_raw_mut(&mut self, index: u32) -> &mut Element<T> {
        self.buffer.get_unchecked_mut(index as usize)
    }

    ///
    ///
    #[inline]
    pub unsafe fn get_unchecked(&self, index: u32) -> Ref<T> {
        Ref::create(index, self as _)
    }

    ///
    ///
    #[inline]
    pub fn get_index(&self, index: u32) -> Option<Ref<T>> {
        if self.buffer.get(index as usize).map_or(false, |element|element.is_used()) {
            Some(unsafe {self.get_unchecked(index)})
        } else {
            None
        }
    }

    ///
    ///
    #[inline]
    pub fn top(&self) -> Ref<T> {
        unsafe {self.get_unchecked(0) }
    }

    ///
    ///
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: u32) -> RefUniq<T> {
        RefUniq::create(index, self as _)
    }

    ///
    ///
    #[inline]
    pub fn get_index_mut(&mut self, index: u32) -> Option<RefUniq<T>> {
        if self.buffer.get(index as usize).map_or(false, |element|element.is_used()) {
            Some(unsafe {self.get_unchecked_mut(index)})
        } else {
            None
        }
    }

    ///
    ///
    #[inline]
    pub fn mut_top(&mut self) -> RefUniq<T> {
        unsafe {self.get_unchecked_mut(0) }
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.buffer.fmt(f)
    }
}