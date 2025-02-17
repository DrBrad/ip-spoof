use std::mem;
use std::os::raw::c_void;
use std::net::Ipv4Addr;
use libc::{AF_INET, IPPROTO_RAW, SOCK_RAW, sockaddr_in, socket, sendto};
use crate::packet::{Packet, TcpPacket, UdpPacket};

pub struct RawSocket {
    src_addr: Ipv4Addr,
    src_port: u16,
}

impl RawSocket {

    pub fn new(src_addr: Ipv4Addr, src_port: u16) -> Self {
        Self {
            src_addr,
            src_port
        }
    }

    pub fn send_udp_packet(&self, addr: Ipv4Addr, port: u16, payload: &[u8], ttl: u8) -> Result<(), String> {
        let socket_fd = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_RAW) };
        if socket_fd < 0 {
            return Err("Failed to create raw socket".to_string());
        }

        let packet = UdpPacket::new(self.src_addr, self.src_port, addr, port, payload, ttl).encode();

        let destination = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from(addr).to_be(),
            },
            sin_zero: [0; 8]
        };

        let send_result = unsafe {
            sendto(
                socket_fd,
                packet.as_ptr() as *const c_void,
                packet.len(),
                0,
                &destination as *const sockaddr_in as *const libc::sockaddr,
                mem::size_of::<sockaddr_in>() as libc::socklen_t
            )
        };

        if send_result < 0 {
            return Err("Failed to send packet".to_string());
        }

        Ok(())
    }

    pub fn send_tcp_packet(&self, addr: Ipv4Addr, port: u16, payload: &[u8], ttl: u8, seq_number: u32, window_size: u16) -> Result<(), String> {
        let socket_fd = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_RAW) };
        if socket_fd < 0 {
            return Err("Failed to create raw socket".to_string());
        }

        let packet = TcpPacket::new(self.src_addr, self.src_port, addr, port, payload, ttl, seq_number, window_size).encode();

        let destination = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from(addr).to_be(),
            },
            sin_zero: [0; 8]
        };

        let send_result = unsafe {
            sendto(
                socket_fd,
                packet.as_ptr() as *const c_void,
                packet.len(),
                0,
                &destination as *const sockaddr_in as *const libc::sockaddr,
                mem::size_of::<sockaddr_in>() as libc::socklen_t
            )
        };

        if send_result < 0 {
            return Err("Failed to send packet".to_string());
        }

        Ok(())
    }
}
