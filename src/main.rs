use std::io::{BufWriter, Write};
use std::net::TcpListener;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);

        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let mut stream = BufWriter::new(stream?);

        stream.write(&"HTTP/1.1 200 OK\n".as_bytes())?;
        stream.write(&"Content-Length: 0\n".as_bytes())?;
        stream.write(&"\n".as_bytes())?;

        stream.flush()?;
    }

    Ok(())
}
