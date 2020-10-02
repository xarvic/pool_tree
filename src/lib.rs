#![feature(arbitrary_self_types)]
#![allow(dead_code)]

pub mod node;
pub mod tree;
pub mod iter;

pub mod prelude{
    pub use crate::{
        node::{Node, NodeMut, NodeTop},
        tree::PoolTree,
    };
}

#[cfg(test)]
mod tests {
    use crate::tree::PoolTree;

    #[test]
    fn test_access() {
        let mut tree = PoolTree::new(17);

        assert_eq!(17, *tree.top());

        **tree.mut_top() = 5;

        assert_eq!(5, *tree.top());
    }

    #[test]
    fn test_alloc() {
        let mut tree = PoolTree::new(17);

        assert_eq!(17, *tree.top());

        **tree.mut_top() = 5;

        assert_eq!(5, *tree.top());
    }

}
