use futures::select;
use futures::FutureExt;
use std::io::{self, BufReader as StdBufReader, BufRead};

use async_std::{
    io::{BufReader},
    net::{TcpStream, ToSocketAddrs},
    prelude::*,
    stream,
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn main() -> Result<()> {
    task::block_on(try_main("127.0.0.1:8080"))
}

async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = (&stream, &stream);
    let reader = BufReader::new(reader);
    let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

    let stdin = StdBufReader::new(io::stdin());
    let mut lines_from_stdin = stream::from_iter(stdin.lines());

    loop {
        select! {
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                },
                None => break,
            },
            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                None => break,
            }
        }
    }
    Ok(())
}
