fn apply_closure<F: Fn(i32, i32) -> i32>(closure: F, x: i32, y: i32) -> i32 {
    closure(x, y)
}

fn main() {
    let z = 30;
    let add_closure = |x, y| {
        println!("global arg [z] is: {z}");
        x + y + z
    };

    let rs = apply_closure(add_closure, 30, 40);
    println!("final result is: {rs}");
}