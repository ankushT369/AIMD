// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;
mod network;

use crate::network::server::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Async server at http://127.0.0.1:7878");
    let port: u16 = 7878;
    Server::bind_addr(port).await?.run().await?;

    Ok(())

    // loop {
    //     let (mut socket, _addr) = listener.accept().await?;
    //     tokio::spawn(async move {
    //         let mut buf = [0u8; 1024];
    //
    //         // Read request (very naive)
    //         match socket.read(&mut buf).await {
    //             Ok(0) => return, // closed
    //             Ok(n) => {
    //                 let req = String::from_utf8_lossy(&buf[..n]);
    //                 let first_line = req.lines().next().unwrap_or_default();
    //
    //                 let (status, body, content_type) = if first_line.starts_with("GET /health") {
    //                     ("HTTP/1.1 200 OK", "ok", "text/plain; charset=utf-8")
    //                 } else if first_line.starts_with("GET / ") || first_line.starts_with("GET /HTTP") || first_line.starts_with("GET / HTTP") {
    //                     ("HTTP/1.1 200 OK", "<h1>Hello from Tokio</h1>", "text/html; charset=utf-8")
    //                 } else if first_line.starts_with("GET ") {
    //                     ("HTTP/1.1 404 Not Found", "<h1>404 Not Found</h1>", "text/html; charset=utf-8")
    //                 } else {
    //                     ("HTTP/1.1 405 Method Not Allowed", "<h1>405</h1>", "text/html; charset=utf-8")
    //                 };
    //
    //                 let headers = format!(
    //                     "{status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
    //                     body.as_bytes().len()
    //                 );
    //                 let _ = socket.write_all(headers.as_bytes()).await;
    //                 let _ = socket.write_all(body.as_bytes()).await;
    //             }
    //             Err(_) => return,
    //         }
    //     });
    // }
}
