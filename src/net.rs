use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    str::FromStr,
    sync::mpsc,
};

use crate::message::{self, Message};
const SERVE_PORT: u16 = 44441;
#[derive(Debug, Clone)]
pub struct SocketList {
    sockets: Vec<SocketAddrV4>,
}

impl SocketList {
    pub fn new(socket_list: &[SocketAddrV4]) -> Self {
        Self {
            sockets: socket_list.to_vec(),
        }
    }
    pub fn new_empty() -> Self {
        Self {
            sockets: Vec::new(),
        }
    }
    pub fn push(&mut self, sock: SocketAddrV4) {
        self.sockets.push(sock);
    }
    pub fn all_as_bytes_v4(&self) -> Vec<u8> {
        let socks = &self.sockets;
        let mut buf = Vec::with_capacity(socks.len() * 6);
        for sock in socks {
            buf.extend(sock.ip().octets());
            buf.extend(sock.port().to_le_bytes());
        }
        buf
    }
    pub fn addrs_and_ports(&self) -> Vec<(&Ipv4Addr, u16)> {
        self.sockets
            .iter()
            .map(|sock| (sock.ip(), sock.port()))
            .collect()
    }
    pub fn sockets(&self) -> &Vec<SocketAddrV4> {
        &self.sockets
    }
}
fn get_listener(port: u16) -> io::Result<TcpListener> {
    let sock = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    TcpListener::bind(sock)
}
pub fn verify_and_accept(
    listener: TcpListener,
) -> io::Result<(TcpStream, message::Init, SocketAddrV4)> {
    let (mut conn, sock) = listener.accept()?;
    let msg = message::Init::recv_and_decode(&mut conn)?;
    let sock = match sock {
        SocketAddr::V4(v4) => v4,
        SocketAddr::V6(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "IPv6 is currently not supported",
            ));
        }
    };
    Ok((conn, msg, sock))
}
pub fn connect(addr: Ipv4Addr, remote_port: u16, local_port: u16) -> io::Result<()> {
    let sock = SocketAddrV4::new(addr, remote_port);
    let mut conn = TcpStream::connect(sock)?;
    let msg = message::Init::new(message::InitKind::New, local_port, "pass", "uname");
    conn.write_all(&msg.encode()?)?;
    let msg = message::ConnsList::recv_and_decode(&mut conn)?;
    connect_to_peers(&msg.consume_for_socket_list(), remote_port)?;
    loop {}
    Ok(())
}
pub fn establish_connection(
    conn: &mut TcpStream,
    sockets: &SocketList,
    init: &message::Init,
) -> io::Result<()> {
    match init.kind() {
        message::InitKind::New => send_socket_list(conn, sockets),
        message::InitKind::Referred => accept_referred(conn),
        message::InitKind::Reconnect => todo!(),
    }
}
pub fn listen(port: u16, send: mpsc::Sender<(TcpStream, String)>) -> io::Result<()> {
    let mut sockets = SocketList::new_empty();

    loop {
        let listener = get_listener(port)?;
        println!("listen: {:?}", listener);
        let (mut conn, init_msg, mut sock) = verify_and_accept(listener)?;
        establish_connection(&mut conn, &sockets, &init_msg)?;
        println!("conn: {}", sock);
        sock.set_port(init_msg.listen_port());
        sockets.push(sock);
        send.send((conn, init_msg.consume_for_username()))
            .expect("Main thread data recv deallocated");
    }
    Ok(())
}
fn send_socket_list(conn: &mut TcpStream, sockets: &SocketList) -> io::Result<()> {
    let msg = message::ConnsList::new(sockets.clone());
    conn.write_all(&msg.encode()?)?;
    Ok(())
}
fn connect_to_peers(sockets: &SocketList, port: u16) -> io::Result<()> {
    for &sock in sockets.sockets() {
        println!("try: {}", sock);
        let mut conn = match TcpStream::connect(sock) {
            Ok(conn) => conn,
            Err(e) => {
                println!("connect_peer: {}", e);
                continue;
            }
        };
        let msg = message::Init::new(message::InitKind::Referred, port, "ref", "refed");
        conn.write_all(&msg.encode()?)?;
    }
    Ok(())
}
fn accept_referred(conn: &mut TcpStream) -> io::Result<()> {
    conn.write_all("hello".as_bytes())
}
