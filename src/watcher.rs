use std::{
    net::{Ipv4Addr, SocketAddrV4},
};

use anyhow::Result;
use serde_json::Value;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::{net::UdpSocket, sync::mpsc::{self, UnboundedReceiver, UnboundedSender}};

use crate::trx::Trx;

const BUFFER_MAX_SIZE: usize = 65535;

#[derive(Debug)]
pub enum MempoolEvent {
    NewTrx(Trx, String),
    NewMsg(Value),
}

pub struct MempoolWatcher {
    multicast_address: String,
}

impl MempoolWatcher {
    pub fn new(multicast_address: &str) -> Self {
        MempoolWatcher {
            multicast_address: multicast_address.into(),
        }
    }
    pub fn start(&self) -> Result<UnboundedReceiver<MempoolEvent>> {
        let (sender, receiver): (UnboundedSender<MempoolEvent>, UnboundedReceiver<MempoolEvent>) = mpsc::unbounded_channel::<MempoolEvent>();
        let udp_socket = self.create_mempool_socket()?;

        tokio::spawn(async move {
            loop {
                let mut buffer = [0; BUFFER_MAX_SIZE];
                if let Ok(_len) = udp_socket.recv(&mut buffer).await {
                    if let Ok(raw_trx) = std::str::from_utf8(&buffer[.._len]) {
                        if let Ok(trx) = serde_json::from_str::<Trx>(&raw_trx.to_string()) {
                            let messages = trx.get_messages();
                            sender
                                .send(MempoolEvent::NewTrx(trx, raw_trx.to_string())).unwrap();
                            for msg in messages.iter().cloned() {
                                sender.send(MempoolEvent::NewMsg(msg)).unwrap();
                            }
                        }
                    }
                }
            }
        });
        Ok(receiver)
    }

    fn create_mempool_socket(&self) -> Result<UdpSocket> {
        let multicast_addr = self
            .multicast_address
            .parse::<SocketAddrV4>()
            .expect("unable to parse multicast address");
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
            .expect("socket creation error");
        socket
            .join_multicast_v4(multicast_addr.ip(), &Ipv4Addr::UNSPECIFIED)
            .expect("join multicast error");
        socket.set_nonblocking(true).expect("nonblocking error");
        socket.set_reuse_address(true).expect("reuse address error");
        socket
            .bind(&socket2::SockAddr::from(multicast_addr))
            .expect("bind error");

        Ok(UdpSocket::from_std(std::net::UdpSocket::from(socket)).unwrap())
    }
}
