#[macro_use]
extern crate log;

use std::io::{BufRead, BufReader, BufWriter, Write};
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
        info!("Read start");

        let stream = stream?;

        let mut reader = BufReader::new(stream);
        let mut buf = String::new();

        loop {
            let size = reader.read_line(&mut buf)?;
            info!("{}", size);
            if buf == "\r\n" || size == 0 {
                break;
            }

            info!("{:?}", buf);

            buf.clear();
        }

        info!("Read end");
        info!("Write start");

        let mut writer = BufWriter::new(reader.into_inner());

        writer.write(&"HTTP/1.1 200 OK\n".as_bytes())?;
        writer.write(&"Content-Length: 0\n".as_bytes())?;
        writer.write(&"\n".as_bytes())?;

        writer.flush()?;

        info!("Write end");
    }

    Ok(())
}
