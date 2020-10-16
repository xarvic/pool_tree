use crate::tree::{Tree, Element};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Receiver};

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
    pub unsafe fn from_inner(inner: RefMut<'a, T>) -> Self {
        RefUniq{
            inner
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

                (&mut*self.buffer).free(child_index)
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