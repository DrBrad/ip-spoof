# IP-Spoofing
IP Spoofing source IP and port with Rust.
You will need to run this as `sudo` for it to work.

> [!WARNING]
> In most countries it is illegal to spoof your IP for nefarious purposes, Do not do this unless you know and have consent with party you are sending the packet to.

Example
------
```rust
use std::net::{Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use ip_spoof::socket::RawSocket;

fn main() {
        let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();

        println!("Listening for incoming UDP packets on port 8080...");

        loop {
            let mut buf = [0; 1024];
            let (len, src) = socket.recv_from(&mut buf).unwrap();
            let payload = &buf[..len];

            println!("Received packet from {}: {}", src, String::from_utf8_lossy(payload));
        }
    });

    thread::sleep(Duration::from_secs(1));

    let socket = RawSocket::new(Ipv4Addr::new(1, 1, 1, 1), 4333);
    socket.send_udp_packet(Ipv4Addr::new(127, 0, 0, 1), 8080, b"hello world", 64).expect("Failed to send packet");

    thread::sleep(Duration::from_secs(1));
}
```
