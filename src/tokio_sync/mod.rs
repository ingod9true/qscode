mod oneshot;
mod lock;
pub mod io;

use tokio::sync::Barrier;
use std::sync::Arc;


async fn xxx(){

    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10));
    for _ in 0..10 {
        let c = barrier.clone();
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        handles.push(tokio::spawn(async move {
            println!("before wait");
            let wait_result = c.wait().await;
            println!("after wait");
            wait_result
        }));
    }

// Will not resolve until all "after wait" messages have been printed
    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

// is_leader 只有一个线程返回true，其他的为false
    assert_eq!(num_leaders, 1);
}

#[test]
fn xx(){

}
