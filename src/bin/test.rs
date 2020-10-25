use pool_tree::tree::Tree;

fn main() {
    let mut tree = Tree::new(7);

    let mut node = tree.mut_top();

    node.add_child(0);
    node.add_child(2);
    node.add_child(4);
    node.add_child(6);

    let mut node = node.get_child_unique(2);

    node.add_child(4);
    node.add_child(4);
    node.add_child(4);
    node.add_child(4);

    println!("tree: {:#?}", tree);
    println!("tree: {}", tree);

}