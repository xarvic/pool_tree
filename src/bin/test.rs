use pool_tree::prelude::*;

fn main() {
    let mut tree = PoolTree::new(7);

    **tree.mut_top() *= 2;

    println!("tree: {}", *tree.top());

    tree.mut_top().add_child(10);
    tree.mut_top().add_child(20);
    tree.mut_top().add_child(30);
    tree.mut_top().add_child(40);

    println!("- {}", *tree.top());

    tree.top().childs().for_each(|child|println!(" - {}", *child));
}