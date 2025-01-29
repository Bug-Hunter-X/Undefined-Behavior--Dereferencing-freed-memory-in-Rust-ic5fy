fn main() {    let mut v = vec![1, 2, 3];    // Create a copy to avoid memory issues
    let v_copy = v.clone();
    let ptr = v_copy.as_ptr();
    let len = v_copy.len();
    unsafe {
        for i in 0..len {
            println!("{}", *ptr.add(i));
        }
    }
    drop(v); // v is dropped here
    drop(v_copy); // v_copy is dropped here
}    