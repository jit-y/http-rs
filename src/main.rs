#[macro_use]
extern crate log;

use std::io::{BufWriter, Write};
use std::net::TcpListener;

fn main() {
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("{}", e);

        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr)?;

    info!("Start server: {}", addr);

    for stream in listener.incoming() {
        let mut stream = BufWriter::new(stream?);

        info!("start");

        stream.write(&"HTTP/1.1 200 OK\n".as_bytes())?;
        stream.write(&"Content-Length: 0\n".as_bytes())?;
        stream.write(&"\n".as_bytes())?;

        stream.flush()?;

        info!("end");
    }

    Ok(())
}
