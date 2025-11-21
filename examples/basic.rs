use glowworm::{Array, array};

fn main() {
    let a = Array::<u64>::zeros(vec![3, 2, 1]);
    println!("{:?}", a);
}
