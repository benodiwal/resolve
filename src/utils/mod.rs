use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig};
use std::net::SocketAddr;

pub const DEFAULT_DNS_SERVER: &str = "1.1.1.1";

pub fn resolver_configuration(ip_str: &str, default: bool) -> ResolverConfig {
    let ip = ip_str.parse().unwrap_or_else(|err| {
        eprintln!("Error parsing IP address: {}", err);
        std::process::exit(1);
    });
    
    let custom_dns_server = NameServerConfig::new(SocketAddr::new(ip, 53), Protocol::Tcp);

    if !default {
        println!("Using custom DNS server: {}", ip_str);
    } else {
        println!("Using default DNS server: {}", ip_str)
    }
            
    ResolverConfig::from_parts(None, Vec::new(), vec![custom_dns_server])
}