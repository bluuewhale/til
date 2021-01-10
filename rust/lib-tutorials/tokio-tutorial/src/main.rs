use futures::prelude::*;
use log::*;
use std::io::Write;
use tokio::prelude::*;
use tokio::task;

type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn our_async_program() -> AsyncResult<String> {
    future::ok("hello World".to_string()).await
}
async fn app() -> AsyncResult<()> {
    // I treat this as the `main` function of the async
    // part of our program.
    let result: String = our_async_program().await?;
    println!("{}", result);

    Ok(())
}

fn init_logger() {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();
}
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:50050/endpoint").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
