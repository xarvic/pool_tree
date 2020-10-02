use smallvec::SmallVec;

fn main() {
    let mut vec = SmallVec::<[u32; 6]>::new();

    vec.push(0);
    vec.push(1);
    vec.push(2);

    println!("vec: {:?}", vec);
}