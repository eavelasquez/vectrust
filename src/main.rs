use vectrust::VectRust;

fn main() {
    let mut vec: VectRust<usize> = VectRust::new();
    vec.push(1usize);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    for i in 0..vec.len() {
        assert_eq!(vec.get(i), Some(&(i + 1)));
    }
    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}
