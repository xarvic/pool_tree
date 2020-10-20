#![feature(receiver_trait)]
#![feature(generic_associated_types)]
#![allow(dead_code)]
#![allow(incomplete_features)]

pub mod node;
pub mod tree;
pub mod ref_unique;
pub mod ref_mut;
pub mod reference;
pub mod ref_global;
mod iter;

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
