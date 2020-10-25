use crate::ref_mut::{RefMut, TreeRefMut};
use crate::iter::ChildIter;
use crate::reference::{TreeRef, Ref};
use crate::tree::{Tree, Element};
use crate::children_unique::ChildrenUnique;
use std::ops::{DerefMut, Deref, Receiver};
use crate::children_mut::ChildrenMut;

pub struct ChildUniq<'a, T>{
    inner: RefMut<'a, T>
}

impl<'a, T: 'static> ChildUniq<'a, T> {
    pub unsafe fn create(buffer: *mut Tree<T>, index: u32) -> Self {
        ChildUniq {
            inner: RefMut::create(index, buffer),
        }
    }
    pub fn get_child_unique(&mut self, index: u32) -> ChildUniq<T> {
        unsafe {
            let index = self.inner.raw().childs().get(index as usize).expect("Index out of Bounds!").get();
            ChildUniq::create(self.inner.buffer, index)
        }
    }
    pub fn get_children_unique(&mut self) -> ChildrenUnique<T> {
        unsafe {
            let indices = self.inner.raw().childs();
            ChildrenUnique::create(self.inner.buffer, indices)
        }
    }
    pub fn get_both_unique(&mut self) -> (&mut T, ChildrenUnique<T>) {
        unsafe {
            let this = self as *mut Self;
            let value = self.inner.deref_mut();
            (value, (&mut*this).get_children_unique())
        }
    }
    pub fn clear_children(&mut self) {
        unsafe{
            let buffer = self.buffer();

            for child_index in self.raw_mut().childs.drain(..) {

                ChildUniq::create(buffer, child_index.get()).clear_children();

                (&mut*buffer).free(child_index);
            }
        }
    }
    pub fn add_child(&mut self, value: T) -> ChildUniq<T> {
        unsafe {
            let index = (*self.buffer()).alloc_for(value, self.index());
            self.raw_mut().children_mut().push(index);

            ChildUniq::create(self.buffer(), index.get())
        }
    }

    pub fn remove_child(&mut self, index: u32) -> T {
        unsafe {
            let buffer = self.buffer();
            let childs = self.raw_mut().children_mut();
            if childs.len() > index as usize {

                let id = childs.remove(index as usize);

                ChildUniq::create(buffer, id.get()).clear_children();

                (&mut*self.buffer()).free(id)
            } else {
                panic!("Index out of Bounds!")
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
        self.inner.buffer
    }

}

impl<'a, T: 'static> Deref for ChildUniq<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}


impl<'a, T: 'static> DerefMut for ChildUniq<'a, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl<'a, T> Receiver for ChildUniq<'a, T> {}

impl<'a, T: 'static> TreeRefMut for ChildUniq<'a, T> {
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

impl<'a, T: 'static> TreeRef for ChildUniq<'a, T> {
    type Type = T;
    type Children<'b> = Ref<'b, T>;

    unsafe fn create(buffer: *const Tree<Self::Type>, index: u32) -> Self {
        Self::create(buffer as *mut Tree<T>, index)
    }

    fn index(&self) -> u32 {
        self.inner.index
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