use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    server().await;
}


async fn server(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        spawn(handle_connection(socket));
    }

}

async fn handle_connection(mut socket: TcpStream){
    let mut buf = [0u8; 1024];
    let size = socket.read(&mut buf).await.unwrap();

    handle_request(&buf[..size]).await;
}

async fn handle_request(buf: &[u8]){
    println!("new request");
}