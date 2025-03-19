use std::{
    io::{self, Read, Write, stdin},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream},
    str::FromStr,
    sync::mpsc,
    thread,
    time::Duration,
    vec,
};

use deck::Deck;
use message::Message;

mod cards;
mod deck;
mod error;
mod message;
mod net;
mod player;
mod translation;
fn main() -> io::Result<()> {
    let mut deck = Deck::new(13);
    deck.shuffle();

    let mut connections: Vec<TcpStream> = vec![];
    let (listen_conns_send, listen_conns_recv) = mpsc::channel();
    let (listen_socks_send, listen_socks_recv) = mpsc::channel();

    println!("port:");
    let mut input_buf = String::new();
    io::stdin().read_line(&mut input_buf)?;
    let local_port: u16 = input_buf.trim().parse().unwrap();
    thread::spawn(move || {
        let res = net::listen(local_port, listen_conns_send, listen_socks_recv);
        println!("listen: {:?}", res);
    });
    let mut input_buf = String::new();
    println!("c:");
    stdin().read_line(&mut input_buf)?;
    let conns = if input_buf.trim() != "" {
        println!("connect");
        net::connect(
            Ipv4Addr::new(127, 0, 0, 1),
            input_buf.trim().parse().unwrap(),
            local_port,
        )
    } else {
        Ok(Vec::new())
    }
    .unwrap_or_default();
    net::send_sockets_to_listen(&listen_socks_send, &conns)?;
    connections.extend(conns);
    loop {
        match listen_conns_recv.try_recv() {
            Ok((conn, username)) => connections.push(conn),
            _ => (),
        };
        println!(
            "{:?}",
            connections
                .iter()
                .filter_map(|e| e.peer_addr().ok())
                .collect::<Vec<_>>()
        );
        let mut input_buf = String::new();
        io::stdin().read_line(&mut input_buf)?;
        for conn in &mut connections {
            conn.write_all(input_buf.as_bytes())?;
            let mut buf = Vec::with_capacity(10);
            conn.read_exact(&mut buf)?;
            println!("{}", String::from_utf8(buf).unwrap());
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
