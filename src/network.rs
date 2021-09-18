use crate::events::ChannelMessage;
use crate::events::NetworkMessage;

use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use fltk::app::Sender;
use std::collections::HashSet;


pub const BRD_PORT: u16 = 34524;

#[derive(Debug)]
pub struct NetworkState {
    set_of_ips: HashSet<Ipv4Addr>,
    local_ip: Ipv4Addr
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            set_of_ips: HashSet::new(),
            local_ip: get_local_ip().unwrap()
        }
    }

    pub fn receive_broadcast_messages(&mut self, port: u16, p_sender: Sender<ChannelMessage>) -> Result<(), Error> {
        let addr = format!("255.255.255.255:{}", port);
        let socket:UdpSocket = UdpSocket::bind(addr)?;
        let mut buf = [0; 10];
        let thread_id = thread::current().id();
        println!("Thread id: {:?}", thread_id);
        loop {
            let (_, src_addr) = socket.recv_from(&mut buf)?;
            println!("recevied address {}", src_addr);
            if src_addr.is_ipv4() {
                let tmp_ip =ipaddr_into_ipv4addr(src_addr.ip()).unwrap();
                self.set_of_ips.insert(tmp_ip);
                let xc = self.set_of_ips.len();
                p_sender.send(ChannelMessage::NM(NetworkMessage { ips_array: hash_set_to_array(&self.set_of_ips)}));
                println!("{:?}", self.set_of_ips);
            }
        }
    }


    pub fn broadcast_ip_and_port(port: u16) -> Result<(), Error> {
        let socket:UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::new(5, 0)))?;
        socket.set_broadcast(true)?;
        socket.connect(("255.255.255.255", port))?;
        println!("Connected on port {}", port);
        println!("Broadcast: {:?}", socket.broadcast());
        println!("Timeout: {:?}", socket.read_timeout());
        loop {
            thread::sleep(Duration::from_millis(2000));
            socket.send(b"test from rust")?;
        }  
    }
}

fn hash_set_to_array(p_set: &HashSet<Ipv4Addr>) -> [Option<Ipv4Addr>; 10] {
    let mut tmp_arr = [None; 10];
    let mut counter = 0;
    for an_elem in p_set {
        tmp_arr[counter] = Some(an_elem.to_owned());
        counter += 1;
    }
    return tmp_arr
}


pub fn get_real_ips_count(ips_array: [Option<Ipv4Addr>; 10], p_local_ip: Ipv4Addr) -> usize {
    let mut counter: usize = 0;
    for ip in ips_array {
        match ip {
            Some(an_ip) => {
                if p_local_ip != an_ip {
                    counter += 1;
                }
            },
            None => {},
        }
    }
    return counter
}

pub fn ipaddr_into_ipv4addr(p_addr: IpAddr) -> Option<Ipv4Addr> {
    match p_addr {
        IpAddr::V4(t_ip) => { Some(t_ip) },
        IpAddr::V6(_) => { None },
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