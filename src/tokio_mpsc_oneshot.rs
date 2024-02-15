use tokio::sync::{mpsc,oneshot};

async fn main(){

    let db:Vec<u32> = vec![1,2,3,4,5];
    let (tx,mut rx) = mpsc::channel::<(u32,oneshot::Sender<bool>)>(10);

    let tx1 = tx.clone();
    let tx2 = tx.clone();


}