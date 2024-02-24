extern crate dns_lookup;

use std::net::IpAddr;
use dns_lookup::{lookup_host};
use std::env;

fn main() {
    //TODO: Extract IPv4 & IPv6 into unique methods that have returns

    // Get domain names from command-line arguments
    let args = env::args().skip(1).collect::<Vec<String>>();

    for domain in args {
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
        }

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

        // Print IPv6 addresses
        for addr in ipv6_addrs {
            println!("{} IPv6 {}", domain, addr);
        }

        println!(); // Add a blank line between domains
    }
}
