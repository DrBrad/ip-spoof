pub mod packet;
pub mod socket;

#[cfg(test)]
mod tests {

    use std::net::Ipv4Addr;

    use crate::socket::RawSocket;

    #[test]
    fn test() {
        let socket = RawSocket::new(Ipv4Addr::new(8, 1, 8, 1), 4333);
        socket.send_udp_packet(Ipv4Addr::new(127, 0, 0, 1), 8080, b"hello world").expect("Failed to send packet");
    }
}