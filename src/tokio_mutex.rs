use std::sync::Arc;
use tokio::sync::{Mutex};

#[tokio::main]
async fn main() {

    let db: Vec<u32> = vec![1, 2, 3, 4, 5];

    // add lock.
    let arc_db = Arc::new(Mutex::new(db));
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move{
        // acquire lock.
        let mut db = arc_db.lock().await;
        db[4] = 50;
        assert_eq!(db[4],50);
    });


    let task_b = tokio::task::spawn(async move{
       let mut db = arc_db2.lock().await;
        db[4] = 100;
        assert_eq!(db[4],100);
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();

    println!("{:?}",arc_db3);

}
