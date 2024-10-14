pub mod packet;
pub mod socket;

#[cfg(test)]
mod tests {

    use std::net::{Ipv4Addr, UdpSocket};
    use std::thread;
    use std::time::Duration;

    use crate::socket::RawSocket;

    #[test]
    fn test() {
        thread::spawn(move || {
            let listener = UdpSocket::bind("0.0.0.0:8080").unwrap();
            let mut buf = [0; 1024];
            loop {
                let (size, src) = match listener.recv_from(&mut buf) {
                    Ok((size, src)) => (size, src),
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        continue;
                    }
                };

                let msg = String::from_utf8_lossy(&buf[..size]);
                println!("Received '{}' from {:?}", msg, src);
            }
        });

        thread::sleep(Duration::from_secs(1));

        let socket = RawSocket::new(Ipv4Addr::new(8, 1, 8, 1), 4333);
        socket.send_to(Ipv4Addr::new(127, 0, 0, 1), 8080, b"hello world").expect("Failed to send packet");

        thread::sleep(Duration::from_secs(1));
    }
}