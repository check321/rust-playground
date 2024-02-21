use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5];

    // init mpsc.back-pressure 10 msg.
    let (tx, mut rx) = mpsc::channel::<u32>(10);

    // mock publishers.
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = tokio::task::spawn(async move {
        if let Err(_) = tx1.send(15).await {
            println!("publish error.");
            return;
        }
    });


    let task_b = tokio::task::spawn(async move {
        if let Err(_) = tx2.send(25).await {
            println!("publish error.");
            return;
        }
    });


    let task_x = tokio::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("receive msg: {}", msg);
            db[4] = msg;
            println!("db {:?}", db);
        }
    });

    // tokio::task::spawn() 这个 API 有个特点，就是通过它创建的异步任务，一旦创建好，就会立即扔到 tokio runtime 里执行，不需要对其返回的 JoinHandler 进行 await 才驱动执行，这个特性很重要。

    // _ = task_a.await.unwrap();
    // _ = task_b.await.unwrap();
    _ = task_x.await.unwrap();
}