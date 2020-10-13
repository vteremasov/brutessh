use std::net::TcpStream;
use std::io::Error;

pub fn scan(host: impl Into<String>, port: impl Into<String>) -> Option<Error> {
    let conn_str = format!("{}:{}", host.into(), port.into());
    
    match TcpStream::connect(conn_str) {
        Ok(_) => None,
        Err(err) => Some(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_scan() {
        match scan("localhost", "1222") {
            None => panic!("should be an error, because 1222 isn't opened"),
            Some(err) => {},
        }
    }
}
 
