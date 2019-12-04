use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

async fn run_tokio() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap_or();
    let res = stream.write_all(b"hello world\n").await;

    println!("wrote to stream, {:?}", res.is_ok());

    Ok(())
}