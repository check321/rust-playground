use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5];
    let (tx, mut rx) = mpsc::channel::<(u32, oneshot::Sender<bool>)>(10);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = task::spawn(async move {
        time::sleep(Duration::from_secs(3)).await;
        // publish/subscribe with ack.
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx1.send((15, resp_tx)).await {
            println!("receiver dropped.");
            return;
        }

        // ack from receiver.
        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_a receive success.");
            } else {
                println!("task_a receive failed.");
            }
        } else {
            println!("task_a receiver dropped.");
            return;
        }
    });


    let task_b = task::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((55, resp_tx)).await {
            print!("receiver dropped.");
            return;
        }

        if let Ok(ret) = resp_rx.await {
            if ret {
                println!("task_b receive success.");
            } else {
                println!("task_b receive failed.");
            }
        } else {
            println!("task_b receiver dropped.");
            return;
        }
    });

    let task_c = task::spawn(async move {
        while let Some((msg, resp_tx)) = rx.recv().await {
            println!("got msg: {}", msg);
            db[4] = msg;
            println!("db: {:?}", db);
            resp_tx.send(true).unwrap();
        }
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    _ = task_c.await.unwrap();
}