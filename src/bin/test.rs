use pool_tree::prelude::*;

fn main() {
    let mut tree = Tree::new(7);

    let mut node = tree.mut_top();

    node.add_child(0);
    node.add_child(2);
    node.add_child(4);
    node.add_child(6);

    node = node.into_child(2).unwrap_or_else(|err|panic!());

    node.add_child(4);

    println!("tree: {:?}", tree);

}