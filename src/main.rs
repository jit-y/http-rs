use std::io::Write;
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
        let mut stream = stream?;

        print!("{:?}", stream);

        let resp = "OK".as_bytes();

        stream.write(&resp)?;
    }

    Ok(())
}
