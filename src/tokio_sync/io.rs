
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufStream;
use tokio::io::{Lines,Result,self};
use std::io::BufRead;
use std::str;
use std::ops::Deref;
use tokio::io::{AsyncRead,AsyncReadExt};

pub async fn dar() -> io::Result<()> {
    let file = File::open("foo.txt").await;
    let mut file = BufReader::new(file.unwrap());
    let mut g = file.buffer();
    let aa = str::from_utf8(g);
    println!("{:?}",aa);
    // let mut lines = file.lines();
    // while let Some(line) = lines.next_line().await? {
    //     println!("length = {}", line.len())
   // }

    let mut g = file.buffer();
    let aa = str::from_utf8(g);
    println!("{:?}",aa);

    Ok(())
}


//net 相关的有 tcplistener tcpsocket tcpstream udpsocket
