use ssh2::{DisconnectCode, Session};
use std::fmt;
use std::net::TcpStream;

#[derive(Clone)]
pub struct Config {
    host: String,
    port: String,
}

pub struct Handler {
    c: Option<Config>,
    t: Option<TcpStream>,
    s: Option<Session>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            host: String::from("localhost"),
            port: String::from("22"),
        }
    }

    pub fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    pub fn set_port(&mut self, port: impl Into<String>) {
        self.port = port.into();
    }

    fn connection_str(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Handler {
    pub fn default() -> Handler {
        Handler {
            c: None,
            t: None,
            s: None,
        }
    }

    pub fn set_config(&mut self, c: Config) {
        self.c = Some(c);
    }

    fn tcp_connect(&mut self) -> Option<Error> {
        match self.c.clone() {
            None => Some(Error::new("invalid config")),
            Some(c) => {
                let cs = c.connection_str();
                let tcp = TcpStream::connect(cs).unwrap();
                self.t = Some(tcp);
                None
            }
        }
    }

    pub fn connect(&mut self) -> Option<Error> {
        match self.t.take() {
            None => match self.tcp_connect() {
                Some(err) => return Some(err),
                None => self.connect(),
            },
            Some(tcp) => {
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.handshake().unwrap();
                self.s = Some(sess);

                None
            }
        }
    }

    pub fn check(&mut self, username: &str, password: &str) -> Result<bool, Error> {
        match self.s.take() {
            None => Err(Error::new("missing session")),
            Some(sess) => {
                match sess.userauth_password(username, password) {
                    Err(_) => Ok(false),
                    Ok(_) => Ok(true),
                }
                //Ok(sess.authenticated())
            }
        }
    }

    pub fn disconnect(&mut self) -> Option<Error> {
        // NOTE: doesn't clost the socket
        let n = match self.s.take() {
            Some(sess) => {
                sess.disconnect(Some(DisconnectCode::AuthCancelledByUser), "no_reason", None)
                    .unwrap();
                None
            }
            None => None,
        };

        n
    }
}

pub struct Error {
    msg: String,
}

impl Error {
    fn new(msg: impl Into<String>) -> Error {
        Error { msg: msg.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_check() {
        let mut h = Handler::default();
        let mut c = Config::default();
        c.set_host("localhost");
        h.set_config(c);

        match h.connect() {
            Some(err) => panic!(err),
            None => {},
        };

        match h.check("root", "nope") {
            Err(err) => panic!(err),
            Ok(v) => println!("{}", v),
        };

        match h.check("root", "nope") {
            Err(err) => panic!(err),
            Ok(v) => println!("{}", v),
        };

        match h.disconnect() {
            Some(err) => panic!(err),
            None => {},
        };

        assert_eq!(2 + 2, 4);
    }
}
