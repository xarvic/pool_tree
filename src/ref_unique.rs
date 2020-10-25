use std::ops::{Receiver, Deref, DerefMut};
use crate::ref_mut::{RefMut, TreeRefMut};
use crate::tree::{Tree, Element};
use crate::reference::{TreeRef, Ref};
use crate::iter::ChildIter;
use crate::children_mut::ChildrenMut;
use crate::child_unique::ChildUniq;

/// RefUniq is an unique Reference to node of the Tree.
/// it has all capabilities of RefMut but additionally can change the structure of the Tree (adding
/// and removing childs of the given Node).
pub struct RefUniq<'a, T> {
    inner: ChildUniq<'a, T>,
}

impl<'a, T: 'static> RefUniq<'a, T> {
    /// create creates a new UniqRef for the Tree buffer to the node at index
    ///
    /// #Safety
    /// The caller must ensure, that no other Ref to the same Tree is accesible during the Lifetime
    /// of this Ref.
    ///
    /// #Example
    /// ```
    /// use pool_tree::node::RefUniq;
    /// pub fn first_child_or_default(value: RefUniq<u32>) -> RefUniq<u32> {
    ///
    /// }
    /// ```
    pub unsafe fn create(index: u32, buffer: *mut Tree<T>) -> Self {
        RefUniq {
            inner: ChildUniq::create(buffer, index)
        }
    }

    /// Convenience Method to create a RefUniq from a RefMut to the same node
    /// see [create]
    pub unsafe fn from_inner(inner: RefMut<T>) -> Self {
        Self::create(inner.index, inner.buffer)
    }
    pub fn inner(self) -> ChildUniq<'a, T> {
        self.inner
    }

    pub fn add_child(&mut self, value: T) -> ChildUniq<T> {
        self.inner.add_child(value)
    }

    pub fn remove_child(&mut self, index: u32) -> T {
        self.inner.remove_child(index)
    }

    pub fn get_child_unique(&mut self, index: u32) -> ChildUniq<T> {
        self.inner.get_child_unique(index)
    }

    pub fn into_parent(self) -> Result<Self, Self> {
        unsafe {
            if self.index() != 0 {
                Ok(RefUniq::create(self.raw().parent(), self.buffer()))
            } else {
                Err(self)
            }
        }
    }

    pub unsafe fn raw(&self) -> &Element<T> {
        self.inner.raw()
    }

    pub unsafe fn raw_mut(&mut self) -> &mut Element<T> {
        self.inner.raw_mut()
    }

    pub unsafe fn raw_index(&self, index: u32) -> &Element<T> {
        self.inner.raw_index(index)
    }

    pub unsafe fn raw_index_mut(&mut self, index: u32) -> &mut Element<T> {
        self.inner.raw_index_mut(index)
    }
    pub(crate) fn buffer(&self) -> *mut Tree<T> {
        self.inner.buffer()
    }

}

impl<'a, T> Receiver for RefUniq<'a, T>{}

impl<'a, T: 'static> Deref for RefUniq<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'a, T: 'static> DerefMut for RefUniq<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl<'a, T: 'static> TreeRefMut for RefUniq<'a, T> {
    fn children_mut(&mut self) -> ChildrenMut<T> {
        self.inner.children_mut()
    }

    fn get_child_mut(&mut self, index: u32) -> RefMut<Self::Type> {
        self.inner.get_child_mut(index)
    }

    fn both(&mut self) -> (&mut Self::Type, ChildrenMut<T>) {
        self.inner.both()
    }
}

impl<'a, T: 'static> TreeRef for RefUniq<'a, T> {
    type Type = T;
    type Children<'b> = Ref<'b, T>;

    unsafe fn create(buffer: *const Tree<Self::Type>, index: u32) -> Self {
        Self::create(index, buffer as *mut Tree<T>)
    }

    fn index(&self) -> u32 {
        self.inner.index()
    }

    fn children<'b>(&'b self) -> ChildIter<'b, Self::Type, Self::Children<'b>> {
        self.inner.children()
    }

    fn get_child<'b>(&'b self, index: u32) -> Self::Children<'b> {
        self.inner.get_child(index)
    }

    fn children_count(&self) -> u32 {
        self.inner.children_count()
    }

    fn get_ref<'b>(&'b self) -> Ref<'b, Self::Type> {
        self.inner.get_ref()
    }
}