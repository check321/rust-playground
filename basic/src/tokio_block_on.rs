fn main() {

// invoke async method in sync method.
    foo();
}

fn foo() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let rs = rt.block_on(bar());
    println!("{}", rs);
}

async fn bar() -> u32 {
    18
}