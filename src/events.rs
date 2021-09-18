use fltk::{prelude::*, window::Window};
use std::{net::Ipv4Addr};

use crate::network;

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
    pub ips_array: [Option<Ipv4Addr>; 10],
}

#[derive(Copy, Clone, Debug)]
pub enum ChannelMessage {
    EM(EventMessage),
    NM(NetworkMessage),
}

pub fn process_network_msg(
    nm: NetworkMessage,
    my_local_ip: Option<Ipv4Addr>,
    tmp_ip: String,
    wind: &mut Window,
) {
    let tmp_ip4 = match my_local_ip {
        Some(ip4) => Some(ip4),
        None => None,
    };
    let ips_count = network::get_real_ips_count(nm.ips_array, tmp_ip4.unwrap());
    let tmp_title = format!("Pist Card Game - {} {}", tmp_ip, ips_count);
    wind.set_label(&tmp_title);
}
