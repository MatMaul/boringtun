use std::{
    net::SocketAddr,
    sync::Arc,
};

use crate::device::udp::UDPSocket;

pub trait NonWireguardHandler: Send {
    fn handle_non_wg_packet(&self, addr: Option<SocketAddr>, packet: &[u8]);
}

pub struct PacketInjector {
    pub udp4: Arc<UDPSocket>,
    pub udp6: Arc<UDPSocket>,
}

impl PacketInjector {
    pub fn inject_packet(&self, addr: SocketAddr, packet: &[u8]) {
        match addr {
            SocketAddr::V4(_) => self.udp4.sendto(packet, addr),
            SocketAddr::V6(_) => self.udp6.sendto(packet, addr),
        };
    }
}
