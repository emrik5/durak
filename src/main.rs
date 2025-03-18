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
mod message;
mod net;
mod player;
mod translation;
fn main() -> io::Result<()> {
    let mut deck = Deck::new(13);
    deck.shuffle();

    // let vec = vec![
    //     SocketAddrV4::from_str("127.0.0.1:1337").unwrap(),
    //     SocketAddrV4::from_str("53.10.0.254:6666").unwrap(),
    // ];
    // let msg = message::ConnsList::new_from_vec(vec);
    // {
    //     let mut f = std::fs::File::create("test")?;
    //     f.write(&msg.encode())?;
    // }
    // let mut f = std::fs::File::open("test")?;
    // let mut buf = vec![];
    // f.read_to_end(&mut buf)?;
    // println!("{:?}", message::ConnsList::decode(&buf));

    let mut connections: Vec<TcpStream> = vec![];
    let (listen_thread_send, listen_thread_recv) = mpsc::channel();

    println!("port:");
    let mut input_buf = String::new();
    io::stdin().read_line(&mut input_buf)?;
    let local_port: u16 = input_buf.trim().parse().unwrap();
    thread::spawn(move || {
        let res = net::listen(local_port, listen_thread_send);
        println!("listen: {:?}", res);
    });
    thread::sleep(Duration::from_secs(5));
    let mut input_buf = String::new();
    println!("c:");
    stdin().read_line(&mut input_buf)?;
    if input_buf.trim() != "" {
        println!("connect");
        net::connect(
            Ipv4Addr::new(127, 0, 0, 1),
            input_buf.trim().parse().unwrap(),
            local_port,
        )?;
        return Ok(());
    }
    loop {
        if listen_thread_recv.try_recv().is_ok() {
            let (conn, username) = listen_thread_recv
                .recv()
                .expect("Listen thread send deallocated");
            connections.push(conn);
        }
        for conn in &mut connections {
            conn.write_all(&"hello".as_bytes())?;
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
