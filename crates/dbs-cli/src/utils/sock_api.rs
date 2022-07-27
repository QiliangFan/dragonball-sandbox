use std::os::unix::net::{UnixListener, UnixStream};
use super::SOCK_PATH;
use std::io::{Error, Write};
use std::process::exit;
use serde_json::{from_str, to_string};


pub fn send(msg: &[u8]) -> Result<(), Error> {
    let mut sock = UnixStream::connect(SOCK_PATH)?;
    sock.write_all(msg)?;
    Ok(())
}

pub fn bind() -> Result<UnixListener, Error>  {
    let mut sock_file = std::path::Path::new(SOCK_PATH);
    if sock_file.exists() {
        match UnixStream::connect(SOCK_PATH) {
            Ok(sc) => {
                println!("Socket is already bound!");
            },
            Err(err) => {
                println!("Redundant socks detected, removing...");
                std::fs::remove_file(sock_file)?;
                let mut soc = UnixListener::bind(SOCK_PATH)?;
                println!("Successfully rebind socket.");
                return Ok(soc);
            }
        };
    }
    // exit(2)
    panic!("ss")
}

fn connect() -> Result<UnixStream, Error> {
    let mut soc = UnixStream::connect(SOCK_PATH)?;
    Ok(soc)
}

