extern crate ssh2;

pub mod ssh;

use std::net::{TcpStream};
use ssh2::{Session, DisconnectCode};
//use ssh::checker;

fn main() {
    let tcp = TcpStream::connect("52.30.167.110:22").unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "Tryh4ckm3").unwrap();
    assert!(sess.authenticated());

    // NOTE: doesn't clost the socket
    sess.disconnect(Some(DisconnectCode::AuthCancelledByUser), "no_reason", None).unwrap();
}
