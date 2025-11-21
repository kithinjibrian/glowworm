use glowworm::array;

fn main() {
    let a = array![1, 2, 3];
    println!("{:?}", a);

    let b = array![[1, 2], [3, 4]];
    println!("{:?}", b);

    let c = array![[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    println!("{:?}", c);
}
