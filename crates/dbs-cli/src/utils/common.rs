use std::path::Path;

pub const SOCK_PATH: &str = "/tmp/dbs/dbs-server.sock";
pub const SOCK_FILE: Path = std::path::Path::new(SOCK_PATH)?;