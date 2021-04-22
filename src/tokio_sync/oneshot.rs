use tokio::sync::oneshot;
use tokio::time::sleep;
use std::time::Duration;

async fn some_computation() -> String {
    sleep(Duration::from_secs(1));
    "represents the result of the computation".to_string()

}

//single producer to a single consumer.
async fn mx(){
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = rx.await.unwrap();
}


use tokio::sync::mpsc;
use std::option::Option::Some;

async fn some_computation_a() -> String {
    sleep(Duration::from_secs(1));
    "represents the result of the computation".to_string()

}

// many values from 多 producers to a single consumer.
async fn mx_a() {
    let (tx, mut rx) = mpsc::channel(10);
    let g = tx.clone();
    tokio::spawn(async move {
       for i in 0..10 {
           let res = some_computation_a().await;
           g.send(res).await.unwrap();
       }
    });
    tokio::spawn(async move {
        for i in 0..30 {
            let res = some_computation_a().await;
            tx.send(res).await.unwrap();
        }
    });

    while  let Some(v) = rx.recv().await{
        println!("got = {}", v);
    }
}

use tokio::sync::broadcast;
use tokio::sync::watch;
async fn mx_b() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();


//watch The watch channel is similar to a broadcast channel with capacity 1.
//	A single-producer, multi-consumer channel that only retains the last sent value.
    //缓存一个值，
    let (ax,bx) = watch::channel(1);
}



