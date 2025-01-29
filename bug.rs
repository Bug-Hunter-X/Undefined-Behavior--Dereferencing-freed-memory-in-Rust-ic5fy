fn main() {    let mut v = vec![1, 2, 3];    let ptr = v.as_mut_ptr();    let len = v.len();    unsafe {        // This is incorrect. It is not guaranteed to be valid after freeing v.
        for i in 0..len {            println!("{}", *ptr.add(i));
        }    }    drop(v); // v is dropped here}