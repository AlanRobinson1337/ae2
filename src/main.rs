extern crate dns_lookup;

use std::net::{IpAddr, SocketAddr, TcpStream};
use dns_lookup::{lookup_host};
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

fn main() {

    let args = env::args().skip(1).collect::<Vec<String>>();
    let ipv4_addrs: Vec<IpDomain> = ipv4(&args);
    let ipv6_addrs: Vec<IpDomain> = ipv6(&args);
    let mut addrs:Vec<IpDomain>= vec![];
    for val in &ipv4_addrs {
        addrs.push(val.clone());
    }
    for val in &ipv6_addrs {
        addrs.push(val.clone());
    }

    println!(); // Add a blank line between domains

    let (conn_client_tx, conn_client_rx) = mpsc::channel();

    // connected client thread
    thread::spawn(move || {
        //println!("{:?}", conn_client_rx.recv());
        let stream = conn_client_rx.recv();
        let mut request_data = String::new();
        request_data.push_str("GET / HTTP/1.1\r\n");
        request_data.push_str(&format!("Host: {:?}\r\n", stream));
        request_data.push_str("Connection: close\r\n");
        request_data.push_str("\r\n");

        println!("{}", request_data)
    });

    let mut flag = true;
    let mut transmitters: Vec<Sender<_>> = vec![];
    //println!("{}", addrs.len());

    for addr in addrs {
        let (conn_attempt_tx, conn_attempt_rx) = mpsc::channel();
        let cloned_tx = conn_client_tx.clone();
        // connection attempt thread
        thread::spawn(move || {
            //println!("hi there");
            let msg = conn_attempt_rx.recv();
            //println!("{:?}", msg);
            cloned_tx.send(format!("{} {}", addr.domain_name, addr.ip_address_port));
        });
        transmitters.push(conn_attempt_tx);

        if flag {
            match TcpStream::connect(addr.ip_address_port) {
                Ok(stream) => {
                    // Connection successful, print connected IP
                    println!("Connected to: {:?}", addr.ip_address_port);
                    println!("{:?}", stream);
                    flag = false;
                    conn_client_tx.send(format!("{:?}", stream));
                }
                Err(_) => {}
            }
        }
    }

    for tx in transmitters {
        tx.send("hi");
       // println!("Hi But Send")
    }

}
#[derive(Debug)]
#[derive(Clone)]
struct IpDomain {
    domain_name: String,
    ip_address:IpAddr,
    ip_address_port:SocketAddr
}

fn add_port(ip_addr: IpAddr) -> SocketAddr{
    let ip_addr_str = ip_addr.to_string();
    let ip_addr = ip_addr_str.parse::<IpAddr>().unwrap();
    let socket_addr = SocketAddr::new(ip_addr, 80);
    return socket_addr
}
fn ipv4(ipv4domains: &Vec<String>) -> Vec<IpDomain> {
    let mut ipv4_addrs_return = vec![];
    for domain in ipv4domains {
        // Perform DNS lookup for IPv4 addresses
        let ipv4_addrs = match lookup_host(&domain) {
            Ok(addrs) => addrs.into_iter()
                .filter(|addr| addr.is_ipv4())
                .collect::<Vec<IpAddr>>(),
            Err(err) => {
                println!("Error: Could not resolve IPv4 addresses for {}: {}", domain, err);
                continue;
            }
        };
        // Print IPv4 addresses
        for addr in ipv4_addrs {
            println!("{} IPv4 {}", domain, addr);
            let con = IpDomain{
                domain_name: domain.clone(),
                ip_address: addr,
                ip_address_port: add_port(addr)
            };
            ipv4_addrs_return.push(con);
        }
        println!(); // Add a blank line between domains
    }
    return ipv4_addrs_return
}
fn ipv6(ipv6domains: &Vec<String>) -> Vec<IpDomain> {
    let mut ipv6_addrs_return = vec![];
    for domain in ipv6domains {
        // Perform DNS lookup for IPv6 addresses
        let ipv6_addrs = match lookup_host(&domain) {
            Ok(addrs) => addrs.into_iter()
                .filter(|addr| addr.is_ipv6())
                .collect::<Vec<IpAddr>>(),
            Err(err) => {
                println!("Error: Could not resolve IPv6 addresses for {}: {}", domain, err);
                continue;
            }
        };
        // Print IPv4 addresses
        for addr in ipv6_addrs {
            println!("{} IPv6 {}", domain, addr);
            let con = IpDomain{
                domain_name: domain.clone(),
                ip_address: addr,
                ip_address_port: add_port(addr)
            };
            ipv6_addrs_return.push(con);
        }
        println!(); // Add a blank line between domains
    }
    return ipv6_addrs_return
}
