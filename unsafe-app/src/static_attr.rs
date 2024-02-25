// global static attr (not recommended.)

static mut COUNTER: u32 = 2023;

fn add_count(inc: u32){
    // unsafe to write.
    unsafe {
        COUNTER += inc;
    }
}

fn main(){

    add_count(1);
    unsafe {
        // unsafe to read.
        println!("COUNTER: {}",COUNTER)
    }
}