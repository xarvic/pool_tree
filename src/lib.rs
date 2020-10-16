#![feature(receiver_trait)]
#![allow(dead_code)]

pub mod node;
pub mod tree;

pub mod prelude{
    pub use crate::{
        node::{Ref, RefMut, RefUniq},
        tree::Tree,
    };
}

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn test_access() {
        let mut tree = Tree::new(17);

        assert_eq!(17, *tree.top());

        **tree.mut_top() = 5;

        assert_eq!(5, *tree.top());
    }

    #[test]
    fn test_alloc() {
        let mut tree = Tree::new(17);

        assert_eq!(17, *tree.top());

        **tree.mut_top() = 5;

        assert_eq!(5, *tree.top());
    }

}
