fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; //modifies the vector
    }
    println!("{:?}", v); 
}