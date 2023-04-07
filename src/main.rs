use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("読み取りエラー: {:?}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("書き込みエラー: {:?}", e);
                    return;
                }
            }
        });
    }
}