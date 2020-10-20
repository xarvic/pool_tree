use std::ops::{Receiver, Deref, DerefMut};
use crate::ref_mut::{RefMut, TreeRefMut};
use crate::tree::Tree;
use crate::reference::{TreeRef, Ref};
use crate::iter::ChildIter;

/// RefUniq is an unique Reference to node of the Tree.
/// it has all capabilities of RefMut but additionally can change the structure of the Tree (adding
/// and removing childs of the given Node).
pub struct RefUniq<'a, T> {
    inner: RefMut<'a, T>,
}

impl<'a, T> RefUniq<'a, T> {
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
            inner: RefMut::create(index, buffer)
        }
    }

    /// Convenience Method to create a RefUniq from a RefMut to the same node
    /// see [create]
    pub unsafe fn from_inner(inner: RefMut<T>) -> Self {
        Self::create(inner.index, inner.buffer)
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
            let buffer = self.buffer;
            let childs = self.raw_mut().children_mut();
            if childs.len() > index as usize {

                let id = childs.remove(index as usize);

                RefUniq::create(id.get(), buffer).clear_children();

                (&mut*self.buffer).free(id)
            } else {
                panic!("Index out of Bounds!")
            }
        }
    }

    pub fn clear_children(&mut self) {
        unsafe{
            let buffer = self.buffer;

            for child_index in self.raw_mut().childs.drain(..) {

                RefUniq::create(child_index.get(), buffer).clear_children();

                (&mut*buffer).free(child_index);
            }
        }
    }

    pub fn create_sub_tree<G: Iterator<Item=(T, G)>>(&mut self, childs_generator: G) {
        for (value, grand_childs) in childs_generator {
            let child = self.add_child(value);

            unsafe { RefUniq::from_inner(child).create_sub_tree(grand_childs); }
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

    pub fn into_new_child(mut self, value: T) -> Self {
        unsafe { RefUniq::from_inner(self.add_child(value)) }
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

impl<'a, T: 'static> TreeRefMut for RefUniq<'a, T> {
    fn children_mut(&mut self) -> ChildIter<Self::Type, RefMut<Self::Type>> {
        self.inner.children_mut()
    }

    fn both(&mut self) -> (&mut Self::Type, ChildIter<Self::Type, RefMut<Self::Type>>) {
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
        self.inner.index
    }

    fn children<'b>(&'b self) -> ChildIter<'b, Self::Type, Self::Children<'b>> {
        self.inner.children()
    }

    fn children_count(&self) -> u32 {
        self.inner.children_count()
    }

    fn get_ref<'b>(&'b self) -> Ref<'b, Self::Type> {
        self.inner.get_ref()
    }
}