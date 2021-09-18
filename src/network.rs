use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use local_ip_address::local_ip;
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

    pub fn receive_broadcast_messages(&mut self, port: u16) -> Result<(), Error> {
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

pub fn get_local_ip() -> Option<Ipv4Addr> {
    match local_ip() {
        Ok(ipa) => { match ipa {
            IpAddr::V4(ip4a) => { Some(ip4a) },
            _ => { None }
        } },
        Err(_) => { None },
    }

}

pub fn ipaddr_into_ipv4addr(p_addr: IpAddr) -> Option<Ipv4Addr> {
    match p_addr {
        IpAddr::V4(t_ip) => { Some(t_ip) },
        IpAddr::V6(_) => { None },
    }
}


