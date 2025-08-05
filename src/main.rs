use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    server().await;
}

async fn server(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();    
    loop {
        if let Ok((socket, _)) = listener.accept().await{
            spawn(handle_connection(socket));
        }
    }
}

async fn handle_connection(mut socket: TcpStream){
    let mut buf = [0u8; 1024];
    loop {
        let size = match socket.read(&mut buf).await {
            Ok(0) => break, // connection closed
            Ok(n) => n,
            Err(_) => break 
        };

        handle_request(&buf[..size]).await;

        if let Err(_) = socket.write(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello").await {
            break;
        }
    }
}

async fn handle_request(buf: &[u8]){
    let _req = std::str::from_utf8(buf).unwrap_or("");
}