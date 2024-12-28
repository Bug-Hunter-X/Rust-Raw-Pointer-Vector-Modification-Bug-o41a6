fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 42; // Safe and correct way to modify the vector
    println!("{:?}", v); 
}