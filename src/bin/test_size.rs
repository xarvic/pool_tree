#![feature(arbitrary_self_types)]

use std::mem::size_of;
use std::num::NonZeroU32;
use pool_tree::tree::Element;

macro_rules! print_size {
    ($test: ty) => {
        println!("size_of::<{}>() = {}", stringify!($test), size_of::<$test>());
    }
}

fn main() {
    print_size!(NonZeroU32);
    print_size!(Option<NonZeroU32>);
    print_size!(bool);
    print_size!(Option<bool>);
    println!();
    print_size!(Element<()>);
    print_size!(Element<u32>);
    print_size!(Element<NonZeroU32>);
    print_size!(Element<String>);
    print_size!(Element<Box<u64>>);
    print_size!(Element<Option<Box<u64>>>);
}