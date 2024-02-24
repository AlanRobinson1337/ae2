extern crate dns_lookup;

use std::net::IpAddr;
use dns_lookup::{lookup_host};
use std::env;

fn main() {
    //TODO: Extract IPv4 & IPv6 into unique methods that have returns

    // Get domain names from command-line arguments
    // let args = env::args().skip(1).collect::<Vec<String>>();

    // for domain in args {
    //     // Perform DNS lookup for IPv4 addresses
    //     let ipv4_addrs = match lookup_host(&domain) {
    //         Ok(addrs) => addrs.into_iter()
    //             .filter(|addr| addr.is_ipv4())
    //             .collect::<Vec<IpAddr>>(),
    //         Err(err) => {
    //             println!("Error: Could not resolve IPv4 addresses for {}: {}", domain, err);
    //             continue;
    //         }
    //     };
    //
    //     // Print IPv4 addresses
    //     for addr in ipv4_addrs {
    //         println!("{} IPv4 {}", domain, addr);
    //     }

        // Perform DNS lookup for IPv6 addresses
        // let ipv6_addrs = match lookup_host(&domain) {
        //     Ok(addrs) => addrs.into_iter()
        //         .filter(|addr| addr.is_ipv6())
        //         .collect::<Vec<IpAddr>>(),
        //     Err(err) => {
        //         println!("Error: Could not resolve IPv6 addresses for {}: {}", domain, err);
        //         continue;
        //     }
        // };
        //
        // // Print IPv6 addresses
        // for addr in ipv6_addrs {
        //     println!("{} IPv6 {}", domain, addr);
        // }
    let args = env::args().skip(1).collect::<Vec<String>>();
    let ipv4_addrs: Vec<IpDomain> = ipv4(args);
    for addr in ipv4_addrs {
        println!(" IPv4 {:?}",  addr);
    }
        println!(); // Add a blank line between domains

    //}
    //cant use ipv_addrs here
}
#[derive(Debug)]
struct IpDomain {
    domain_name: String,
    ipv4: bool,
    ipv6: bool,
    ip_address:IpAddr
}
fn ipv4(ipv4domains: Vec<String>) -> Vec<IpDomain> {
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
                ipv4: true,
                ipv6: false,
                ip_address: addr,
            };
            ipv4_addrs_return.push(con);
        }
        println!(); // Add a blank line between domains
    }
    return ipv4_addrs_return
}
fn ipv6(ipv6domains: Vec<String>) -> Vec<IpDomain> {
    let mut ipv6_addrs_return = vec![];
    for domain in ipv6domains {
        // Perform DNS lookup for IPv4 addresses
        let ipv6_addrs = match lookup_host(&domain) {
            Ok(addrs) => addrs.into_iter()
                .filter(|addr| addr.is_ipv4())
                .collect::<Vec<IpAddr>>(),
            Err(err) => {
                println!("Error: Could not resolve IPv4 addresses for {}: {}", domain, err);
                continue;
            }
        };
        // Print IPv4 addresses
        for addr in ipv6_addrs {
            println!("{} IPv4 {}", domain, addr);
            let con = IpDomain{
                domain_name: domain.clone(),
                ipv4: false,
                ipv6: true,
                ip_address: addr,
            };
            ipv6_addrs_return.push(con);
        }
        println!(); // Add a blank line between domains
    }
    return ipv6_addrs_return
}
