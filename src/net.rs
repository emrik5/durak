use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};
const SERVE_PORT: u16 = 44441;

pub fn listen() -> io::Result<TcpStream> {
    let sock = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, SERVE_PORT);
    let listener = TcpListener::bind(sock)?;
}
pub fn verify_and_connect(conn: TcpListener, data: String) -> io::Result<TcpStream> {}
