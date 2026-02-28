use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::net::SocketAddr;


// Server struct holds the socket addres and 
// TCP listener instance for reuse purposes
pub struct Server {
    sock_addr: SocketAddr,
    listener: TcpListener,
}

// Server implementations
impl Server {
    // bind address to server
    pub async fn bind_addr(port: u16) -> std::io::Result<Self> {
        let sock_addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(sock_addr).await?;
        Ok(Self { sock_addr, listener })
    }

    pub async fn run(self) -> std::io::Result<()> {
        println!("Listening on {}", self.listener.local_addr()?);

        loop {
            let (mut socket, _addr) = self.listener.accept().await?;
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        let req = String::from_utf8_lossy(&buf[..n]);
                        println!("req: \n{}", req);
                        let first_line = req.lines().next().unwrap_or_default();
                        println!("first line: \n{}", first_line);

                        let (status, body, content_type) = if first_line.starts_with("POST /health") {
                            ("HTTP/1.1 200 OK", "{ Agent ID: aed33234, token: 435, expiration_time: 3000}", "text/html; charset=utf-8")
                        } else if first_line.starts_with("GET / ") || first_line.starts_with("GET /HTTP") || first_line.starts_with("GET / HTTP") {
                            ("HTTP/1.1 200 OK", "Agent ID: aed33234", "text/html; charset=utf-8")
                        } else if first_line.starts_with("GET ") {
                            ("HTTP/1.1 404 Not Found", "<h1>404 Not Found</h1>", "text/html; charset=utf-8")
                        } else {
                            ("HTTP/1.1 405 Method Not Allowed", "<h1>405</h1>", "text/html; charset=utf-8")
                        };

                        let headers = format!(
                            "{status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.as_bytes().len()
                        );
                        let _ = socket.write_all(headers.as_bytes()).await;
                        let _ = socket.write_all(body.as_bytes()).await;
                    }
                    Err(_) => return,
                }
            });
        }
    }
}
