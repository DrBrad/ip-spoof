use std::mem;
use std::net::{Ipv4Addr, SocketAddr};

#[repr(C)]
struct IpHeader {
    version_ihl: u8,
    tos: u8,
    length: u16,
    id: u16,
    flags_offset: u16,
    ttl: u8,
    protocol: u8,
    checksum: u16,
    src_ip: u32,
    dst_ip: u32,
}

#[repr(C)]
struct UdpHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

pub struct Packet {
    src_addr: Ipv4Addr,
    src_port: u16,
    dst_addr: Ipv4Addr,
    dst_port: u16,
    payload: Vec<u8>
}

impl Packet {

    pub fn new(src_addr: Ipv4Addr, src_port: u16, dst_addr: Ipv4Addr, dst_port: u16, payload: &[u8]) -> Self {
        Self {
            src_addr,
            src_port,
            dst_addr,
            dst_port,
            payload: payload.to_vec()
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let total_length = 20 + 8 + self.payload.len(); // IP header (20 bytes) + UDP header (8 bytes)

        let ip_header = IpHeader {
            version_ihl: (4 << 4) | 5, // IPv4, header length 5 (20 bytes)
            tos: 0,
            length: (total_length as u16).to_be(),
            id: 0,
            flags_offset: 0,
            ttl: 64,
            protocol: libc::IPPROTO_UDP as u8,
            checksum: 0,
            src_ip: u32::from(self.src_addr).to_be(),
            dst_ip: u32::from(self.dst_addr).to_be(),
        };

        let udp_length = (8 + self.payload.len()) as u16;
        let udp_header = UdpHeader {
            src_port: self.src_port.to_be(),
            dst_port: self.dst_port.to_be(),
            length: udp_length.to_be(),
            checksum: 0
        };

        let mut packet = Vec::with_capacity(total_length);
        let ip_header_bytes: [u8; 20] = unsafe { mem::transmute(ip_header) };
        let udp_header_bytes: [u8; 8] = unsafe { mem::transmute(udp_header) };

        packet.extend_from_slice(&ip_header_bytes);
        packet.extend_from_slice(&udp_header_bytes);
        packet.extend_from_slice(self.payload.as_slice());

        let ip_checksum = self.checksum(&packet[..20]);
        packet[10..12].copy_from_slice(&ip_checksum.to_be_bytes());

        packet
    }

    fn checksum(&self, data: &[u8]) -> u16 {
        let mut sum = 0u32;
        let len = data.len();

        for i in (0..len).step_by(2) {
            let word = (data[i] as u32) << 8 | (data[i + 1] as u32);
            sum = sum.wrapping_add(word);
        }

        while (sum >> 16) != 0 {
            sum = (sum & 0xffff) + (sum >> 16);
        }

        !sum as u16
    }
}
