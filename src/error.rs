use std::io;

pub fn ipv6_error() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "IPv6 is currently not supported.",
    ))
}
