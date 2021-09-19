extern crate serde;

use crate::events::ChannelMessage;
use crate::events::NetworkMessage;
use crate::game::Card;

use fltk::app::Sender;
use std::collections::HashSet;
use std::io::Error;
use std::io::Read;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub const BRD_PORT: u16 = 34524;

#[derive(Debug)]
pub struct NetworkState {
    set_of_ips: HashSet<Ipv4Addr>,
    local_ip: Option<Ipv4Addr>,
}

#[derive(Serialize, Copy, Clone, Debug, Deserialize)]
pub enum GameNetworkMsg {
    PlayedCard(Card),
    InviteRequest,
    Accepted(bool),
}

#[derive(Serialize, Copy, Clone, Debug, Deserialize)]
pub struct TcpMsg {
    game_network_msg: Option<GameNetworkMsg>,
}

impl TcpMsg {
    pub fn new() -> Self {
        Self {
            game_network_msg: None,
        }
    }
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            set_of_ips: HashSet::new(),
            local_ip: None,
        }
    }

    pub fn receive_broadcast_messages(
        &mut self,
        port: u16,
        p_sender: Sender<ChannelMessage>,
    ) -> Result<(), Error> {
        let addr = format!("255.255.255.255:{}", port);
        let socket: UdpSocket = UdpSocket::bind(addr)?;
        let mut buf = [0; 10];
        let thread_id = thread::current().id();
        println!("Thread id: {:?}", thread_id);
        loop {
            let (_, src_addr) = socket.recv_from(&mut buf)?;
            // println!("recevied address {}", src_addr);
            if src_addr.is_ipv4() {
                let tmp_ip = ipaddr_into_ipv4addr(src_addr.ip()).unwrap();
                self.set_of_ips.insert(tmp_ip);
                p_sender.send(ChannelMessage::NM(NetworkMessage {
                    ips_array: hash_set_to_array(&self.set_of_ips),
                }));
                println!("{:?}", self.set_of_ips);
            }
        }
    }

    pub fn receive_invite_request(&mut self, port: u16) -> Result<(), Error> {
        let addr = format!("127.0.0.1:{}", port);
        let mut stream = TcpStream::connect(addr)?;
        let x = TcpMsg::new();
        let mut buf = bincode::serialize(&x).unwrap();
        match stream.read_exact(&mut buf) {
            Ok(_) => {
                let y: TcpMsg = bincode::deserialize(&buf).unwrap();
                println!("incoming msg: {:?}", y)
            }
            Err(an_er) => {
                print!("error: {}", an_er)
            }
        }
        // stream.read_exact(buf)
        // stream.write(&[1])?;
        // stream.read(&mut [0; 128])?;
        Ok(())
    }

    pub fn wait_for_remote_connect(port: u16) {
        let sc_ard_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        if let Ok(stream_2) = TcpListener::bind(sc_ard_2) {
            println!("Connected to the server! {}", sc_ard_2);
            for stream in stream_2.incoming() {
                println!("new client: {:?}", stream);
                // handle_client(stream?);
            }
        } else {
            println!("Couldn't connect to server... {}", sc_ard_2);
        }
    }

    pub fn broadcast_ip_and_port(port: u16) -> Result<(), Error> {
        // let local_ip = get_local_ip().unwrap();
        // let s_addr = SocketAddrV4::new(local_ip, 0);
        let socket: UdpSocket = UdpSocket::bind("127.0.0.1:0")?;
        socket.set_read_timeout(Some(Duration::new(5, 0)))?;
        socket.set_broadcast(true)?;
        socket.connect(("255.255.255.255", port))?;
        println!("Connected on port {}", port);
        println!("Broadcast: {:?}", socket.broadcast());
        println!("Timeout: {:?}", socket.read_timeout());
        loop {
            thread::sleep(Duration::from_millis(2000));
            // println!("broadcast socket: {:?}", socket);
            socket.send(b"test from rust")?;
        }
    }
}

fn hash_set_to_array(p_set: &HashSet<Ipv4Addr>) -> [Option<Ipv4Addr>; 10] {
    let mut tmp_arr = [None; 10];
    let mut counter = 0;
    for an_elem in p_set {
        if counter < 10 {
            tmp_arr[counter] = Some(an_elem.to_owned());
        }
        counter += 1;
    }
    return tmp_arr;
}

pub fn get_real_ips_count(ips_array: [Option<Ipv4Addr>; 10], p_local_ip: Ipv4Addr) -> usize {
    let mut counter: usize = 0;
    for ip in ips_array {
        match ip {
            Some(an_ip) => {
                if p_local_ip != an_ip {
                    counter += 1;
                }
            }
            None => {}
        }
    }
    return counter;
}

pub fn ipaddr_into_ipv4addr(p_addr: IpAddr) -> Option<Ipv4Addr> {
    match p_addr {
        IpAddr::V4(t_ip) => Some(t_ip),
        IpAddr::V6(_) => None,
    }
}

/// get the local ip address, return an `Option<String>`. when it fail, return `None`.
pub fn get_local_ip() -> Option<Ipv4Addr> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(ipaddr_into_ipv4addr(addr.ip()).unwrap()),
        Err(_) => return None,
    };
}

pub fn send_invite_request(_my_ip: Ipv4Addr, his_ip: Ipv4Addr, port: u16) -> Result<usize, Error> {
    println!("trying to send invite to address: {}", his_ip);
    let _sc_adr = SocketAddrV4::new(his_ip, port);
    let sc_adr_2 = SocketAddr::new(IpAddr::V4(his_ip), port);
    if let Ok(_stream_2) = TcpStream::connect_timeout(&sc_adr_2, Duration::from_secs(2)) {
        println!("Connected to the server! {}", his_ip);
    } else {
        println!("Couldn't connect to server... {}", his_ip);
    }
    Ok(4)
    // println!("trying to send invite to address: {}", sc_adr);
    // let mut stream = match TcpStream::connect(sc_adr) {
    //     Ok(a_s) => {Some(a_s)},
    //     Err(e) => {println!("error: {}", e); None},
    // }.unwrap();
    // stream.set_write_timeout(Some(Duration::from_secs(2))).expect("Cannot set timeout");
    // let yyyyy = stream.write_timeout().unwrap().unwrap().as_secs();
    // println!("timeout: {}", yyyyy);
    // let tcpmsg=TcpMsg{
    //     game_network_msg: Some(GameNetworkMsg::InviteRequest),
    // };
    // let buf = bincode::serialize(&tcpmsg).unwrap();
    // match stream.write(&buf) {
    //     Ok(nn) => {println!("{} bytes written", nn); Ok(nn)},
    //     Err(e) => {println!("error e: {}", e); Err(e)},
    // }
}
