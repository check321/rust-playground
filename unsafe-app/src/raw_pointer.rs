// raw pointer: *const T / *mut T
fn main(){
    let num_1 = 20;
    let num_1_ptr: *const i32 = &num_1;

    let mut num_2 = 24;
    let num_2_ptr: *mut i32 = &mut num_2;

    unsafe {
        println!("num_1: {}",*num_1_ptr);
        println!("num_2: {}",*num_2_ptr);
    }

}