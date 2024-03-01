fn main() {
    // Fn: read only.
    let s_1 = "hola Rust.".to_string();
    let fn_func = |s| {
        println!("global arg: {s_1}");
        println!("closure arg: {s}")
    };
    fn_func("2024".to_string());


    // FnMut: read and write. &mut likely.
    let mut s_2 = "greetings Rust.".to_string();
    let mut fn_mut_func = |s| {
        s_2.push_str(s);
        println!("global args: {s_2}");
    };
    fn_mut_func("2024");

    // FnOnce: ownership moving.
    let s_3 = String::from("你好，Rust");
    let fn_once_func  = || {
        println!("global args: {s_3}");
        std::mem::drop(s_3);
        // err: value moved.
        // println!("global args: {s_3}");
    };
    fn_once_func();
}