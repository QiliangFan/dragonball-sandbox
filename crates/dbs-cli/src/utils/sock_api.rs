use std::os::unix::net::{UnixListener, UnixStream};
use super::SOCK_PATH;
use std::io::{Error, Write};
use crate::utils::SOCK_FILE;

pub fn send(msg: u8, time_out: Option<u32>) {

}


pub fn bind() -> Result<UnixListener, Error> {
    if SOCK_FILE.exists() {
        let mut sniffer = UnixStream::connect(SOCK_PATH)?;
        sniffer.set_write_timeout(std::time::Duration::from_millis(2000)?)?;
        match sniffer.write_all(b"Are you ok?") {
            Ok(soc) => {
                println!("Rebinding to existing socket channel...")
            }
            Err(err) => {
                println!("Redundant socks detected => removing");
                std::fs::remove_file(SOCK_FILE)?;
            }
        }
    }

    let mut soc = UnixListener::bind(SOCK_PATH)?;
    Ok(soc)
}


fn connect() -> Result<UnixStream, Error> {
    let mut soc = UnixStream::connect(SOCK_PATH)?;
    Ok(soc)
}

