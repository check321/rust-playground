use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let lock = RwLock::new(7);
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;

        assert_eq!(*r1, 7);
        assert_eq!(*r2, 7);
    }

    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w,8);
    }
}