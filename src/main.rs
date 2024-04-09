use std::io::Write;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
mod task_manager;
use task_manager::ThreadPool;
enum ServerType {
    Tokio,
    ThreadPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <server_type> <num_threads>");
        std::process::exit(1);
    }

    let num_threads = match args[2].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number of threads. Please provide a positive integer.");
            std::process::exit(1);
        }
    };

    let server_type = match args[1].as_str() {
        "tokio" => ServerType::Tokio,
        "threadpool" => ServerType::ThreadPool,
        _ => {
            eprintln!("Invalid server type. Choose 'tokio' or 'threadpool'.");
            std::process::exit(1);
        }
    }; // Change this to ServerType::Tokio if you want to use Tokio

    match server_type {
        ServerType::Tokio => {
            start_server(addr, num_threads).await?;
        }
        ServerType::ThreadPool => {
            start_threadpool_server(addr, num_threads).expect("Failed to start thread pool server");
        }
    }

    Ok(())
}

async fn start_server(
    addr: SocketAddr,
    num_threads: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Tokio server running on {}", addr);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_threads)
        .enable_all()
        .build()?;
    loop {
        let (socket, _) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        runtime.spawn(async move {
            if let Err(e) = handle_connection_tokio(socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    // Placeholder for other server types
}

fn start_threadpool_server(addr: SocketAddr, num_threads: usize) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(addr)?;
    let pool = ThreadPool::new(num_threads);

    println!("Threadpool server running on {}", addr);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        pool.execute(move || {
            handle_connection_threadpool(stream);
        });
    }

    Ok(())
}

async fn handle_connection_tokio(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    pi_bbp(100000);
    println!("Done");
    socket.write_all(response.as_bytes()).await?;
    Ok(())
}

fn handle_connection_threadpool(mut stream: std::net::TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    pi_bbp(100000);
    let _ = stream.write_all(response.as_bytes());
}

fn pi_bbp(n: usize) -> f64 {
    let mut pi: f64 = 0.0;
    for k in 0..n {
        pi += ((4.0 / (8 * k + 1) as f64)
            - (2.0 / (8 * k + 4) as f64)
            - (1.0 / (8 * k + 5) as f64)
            - (1.0 / (8 * k + 6) as f64))
            / 16.0_f64.powi(k as i32);
    }
    pi
}
