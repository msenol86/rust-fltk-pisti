use std::{collections::HashSet, net::Ipv4Addr};

#[derive(Copy, Clone, Debug)]
pub enum Player {
    Player1,
    Player2,
}


#[derive(Copy, Clone, Debug)]
pub struct EventMessage {
    pub card_index: u8,
    pub the_player: Player,
}

#[derive(Copy, Clone, Debug)]
pub struct NetworkMessage {
    pub ips_array: [Option<Ipv4Addr>; 10]
}


#[derive(Copy, Clone, Debug)]
pub enum ChannelMessage {
    EM(EventMessage),
    NM(NetworkMessage)
}