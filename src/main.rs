
mod error;
mod body;
mod example;
mod tokio_sync;
use tokio_sync::io::dar;
use tokio::time::{self,Duration};
use example::ex_pin::Demo;
use std::pin::Pin;
use tokio::sync::oneshot;
use std::borrow::Borrow;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::io;
use tokio::fs::DirBuilder;
use tokio::fs;
use tokio::time::sleep;
use futures::stream::AndThen;
use futures::stream::unfold;
use futures::stream::Stream;
//tokio 主要使用的是io net sync


#[tokio::main]
async fn main() -> io::Result<()> {
    tokio_normal_use();


    dar();


    let mut entries = fs::read_dir(".").await?;
    while let Some(entry) = entries.next_entry().await? {
        println!("{:?}", entry.path());
    }
    Ok(())
}



async fn tokio_normal_use(){
    let mut name = String::from("shang");
    tokio::spawn(async move {
        name.push_str("zhang");
        println!("hello shang! {}",name);
    });

    println!("hello world!");

    let mut a = Demo{all:32,ins:"sz"};
    Pin::new(&mut a).method();


    let (tx1, mut rx1) = oneshot::channel();
    let (tx2, mut rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("first").unwrap();
    });

    tokio::spawn(async move {
        tx2.send("second").unwrap();
    });
    let mut a = None;
    let mut b = None;
    while a.is_none() || b.is_none() {
        tokio::select! {
            v1 = (&mut rx1), if a.is_none() => a = Some(v1.unwrap()),
            v2 = (&mut rx2), if b.is_none() => b = Some(v2.unwrap()),
        }
    }
    let res = (a.unwrap(), b.unwrap());
    assert_eq!(res.0, "first");
    assert_eq!(res.1, "second");

    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = more_async_work() => {
            println!("more_async_work() completed first")
        }
    };

    // let sleep = time::sleep(Duration::from_millis(5));
    // tokio::pin!(sleep);
    // loop {
    //     tokio::select! {
    //         _ = &mut sleep => {
    //             println!("operation timed out");
    //             break;
    //         }
    //         _ = more_async_work() => {
    //             println!("operation completed");
    //         }
    //     }
    // }
    //try_join!
    let res = tokio::try_join!(do_stuff_asynca(),more_async_worka());
    match res {
        Ok((firsts,second)) => {},
        Err(err) => {
            println!("{}",err);
        }
    }

}

const CONST_LIFETIME_MEEP: &'static str = "MEEP";
async fn do_stuff_asynca() -> Result<(), &'static str> {
    // async work
    Ok(())

}

async fn more_async_worka() -> Result<(), &'static str> {
    // more here
    Err(CONST_LIFETIME_MEEP)
}


async fn do_stuff_async() {
    // async work
    sleep(Duration::from_millis(100)).await;
}

async fn more_async_work() {
    // more here
}


// use std::convert::Infallible;
// use pin_project::pin_project;
// use std::pin::Pin;
// extern crate  hyper;
//
// use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response,Server};
//
// async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello World!")))
// }

// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     pretty_env_logger::init();
//
//     let make_svc = make_service_fn(|_conn| {
//         // This is the `Service` that will handle the connection.
//         // `service_fn` is a helper to convert a function that
//         // returns a Response into a `Service`.
//         async { Ok::<_, Infallible>(service_fn(hello)) }
//     });
//
//     let addr = ([127, 0, 0, 1], 3000).into();
//
//     let server = Server::bind(&addr).serve(make_svc);
//
//     println!("Listening on http://{}", addr);
//
//     server.await?;
//
//     Ok(())
// }

