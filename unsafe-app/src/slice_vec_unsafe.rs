use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    // splice two mut vec.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main(){
    let mut vec = vec![1,3,5,2,4,6];
    let (left, right) = split_at_mut(&mut vec,3);
    println!("left vec: {:?}, right vec: {:?}",left,right);

}