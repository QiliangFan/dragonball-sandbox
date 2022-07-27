use std::os::unix::net::{UnixListener, UnixStream};
use super::SOCK_PATH;
use std::io::{Error, Write};

pub fn send(msg: u8, time_out: Option<u32>) {

}


pub fn bind() -> Result<UnixListener, Error>  {
    let mut sock_file = std::path::Path::new(SOCK_PATH);
    if sock_file.exists() {
        let mut sniffer = UnixStream::connect(SOCK_PATH)?;
        let timeout = std::time::Duration::from_millis(2000);
        sniffer.set_write_timeout(Option::Some(timeout))?;
        match sniffer.write_all(b"Are you ok?") {
            Ok(soc) => {
                println!("Rebinding to existing socket channel...")
            }
            Err(err) => {
                println!("Redundant socks detected => removing");
                std::fs::remove_file(sock_file)?;
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

