use std::net::TcpStream;

use crate::cards::Hand;

struct Player {
    name: String,
    conn: TcpStream,
    hand: Hand,
}
