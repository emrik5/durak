use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD as BASE64_URL_SAFE};

use crate::net;

const DATA_SEPARATOR: u8 = '$' as u8;
pub trait Message<T: Sized> {
    fn recv_and_decode(conn: &mut impl Read) -> io::Result<T>;
    fn encode(&self) -> io::Result<Vec<u8>>;
    fn decode(data: &[u8]) -> Option<T>;
}
enum MessageKind {
    Init = 1,
    ConnsList = 2,
}
#[derive(Debug, Clone, Copy)]
pub enum InitKind {
    New = 1,
    Referred = 2,
    Reconnect = 3,
}
impl InitKind {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::New),
            2 => Some(Self::Referred),
            3 => Some(Self::Reconnect),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct Init {
    kind: InitKind,
    listen_port: u16,
    auth: String,
    username: String,
}

impl Init {
    pub fn new(kind: InitKind, listen_port: u16, auth: &str, username: &str) -> Self {
        let auth = auth.to_string();
        let username = username.to_string();
        Self {
            kind,
            listen_port,
            auth,
            username,
        }
    }
    pub fn kind(&self) -> InitKind {
        self.kind
    }
    pub fn listen_port(&self) -> u16 {
        self.listen_port
    }
    pub fn consume_for_username(self) -> String {
        self.username
    }
}
impl Message<Init> for Init {
    fn recv_and_decode(conn: &mut impl Read) -> io::Result<Self> {
        let mut len = [0u8; 2];
        conn.read_exact(&mut len)?;
        let len = u16::from_le_bytes(len);
        let mut buf = vec![0u8; len as usize];
        conn.read_exact(&mut buf)?;
        Init::decode(&buf).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Init msg decode fail",
        ))
    }
    /// Encode message as bytes, using URL-safe base64 for strings
    fn encode(&self) -> io::Result<Vec<u8>> {
        let kind = self.kind as u8;
        let port = self.listen_port;
        let auth = BASE64_URL_SAFE.encode(self.auth.clone()).into_bytes();
        let username = BASE64_URL_SAFE.encode(self.username.clone()).into_bytes();
        // msg_kind + kind + port + auth + DATA_SEPARATOR + username
        let len = 1 + 1 + 2 + auth.len() + 1 + username.len();
        let len: u16 = match len.try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Init message too long",
                ));
            }
        };
        let mut ret = len.to_le_bytes().to_vec();
        ret.extend([MessageKind::Init as u8, kind]);
        ret.extend(port.to_le_bytes());
        ret.extend(auth);
        ret.push(DATA_SEPARATOR);
        ret.extend(username);
        Ok(ret)
    }
    /// Decode message from bytes
    fn decode(data: &[u8]) -> Option<Self> {
        if *data.first()? != MessageKind::Init as u8 {
            return None;
        }
        let kind = *data.get(1)?;
        let listen_port = data.get(2..=3)?.try_into().ok()?;
        let listen_port = u16::from_le_bytes(listen_port);
        let [auth, username] = data[4..]
            .split(|&c| c == DATA_SEPARATOR)
            .map(|encoded| {
                BASE64_URL_SAFE
                    .decode(encoded)
                    .ok()
                    .and_then(|str| String::from_utf8(str).ok())
            })
            .collect::<Option<Vec<String>>>()?
            .try_into()
            .ok()?;
        Some(Self {
            kind: InitKind::from_u8(kind)?,
            listen_port,
            auth,
            username,
        })
    }
}
#[derive(Debug)]
pub struct ConnsList(net::SocketList);

impl ConnsList {
    pub fn new(socket_list: net::SocketList) -> Self {
        Self(socket_list)
    }
    pub fn new_from_vec(socket_list: Vec<SocketAddrV4>) -> Self {
        Self(net::SocketList::new(&socket_list))
    }
    pub fn new_empty() -> Self {
        Self(net::SocketList::new_empty())
    }
    pub fn consume_for_socket_list(self) -> net::SocketList {
        self.0
    }
}

impl Message<ConnsList> for ConnsList {
    fn recv_and_decode(conn: &mut impl Read) -> io::Result<Self> {
        let mut len = [0u8; 2];
        conn.read_exact(&mut len)?;
        let len = u16::from_le_bytes(len);
        let mut buf = vec![0u8; len as usize];
        conn.read_exact(&mut buf)?;
        ConnsList::decode(&buf).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "ConnsList msg decode fail",
        ))
    }

    fn encode(&self) -> io::Result<Vec<u8>> {
        let socks = self.0.all_as_bytes_v4();
        let len = 1 + socks.len();
        let len: u16 = match len.try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "ConnsList message too long",
                ));
            }
        };
        let mut buf = len.to_le_bytes().to_vec();
        buf.push(MessageKind::ConnsList as u8);
        buf.extend(self.0.all_as_bytes_v4());
        Ok(buf)
    }

    fn decode(data: &[u8]) -> Option<Self> {
        if *data.first()? != MessageKind::ConnsList as u8 {
            return None;
        }
        if data.len() == 1 {
            return Some(ConnsList::new_empty());
        }
        let vec: Vec<SocketAddrV4> = data[1..]
            .chunks(6)
            .map(|chunk| {
                let [a, b, c, d, p1, p2] = chunk.try_into().unwrap_or_default();
                SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), u16::from_le_bytes([p1, p2]))
            })
            .collect();
        Some(ConnsList::new_from_vec(vec))
    }
}
