use std::net::SocketAddr;

use trust_dns_resolver::{config::ResolverConfig, Resolver};
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverOpts};

pub fn resolve(domain: &str, dns_server: Option<&str>) {
    let resolver_config = match dns_server {
        Some(ip_str) => {
            
            let ip = ip_str.parse().unwrap_or_else(|err| {
                eprintln!("Error parsing IP address: {}", err);
                std::process::exit(1);
            });

            let custom_dns_server = NameServerConfig::new(SocketAddr::new(ip, 53), Protocol::Tcp);
            ResolverConfig::from_parts(None, Vec::new(), vec![custom_dns_server])
        }

        None => ResolverConfig::default(),
    };

    let resolver_opts = ResolverOpts::default();
    let resolver = Resolver::new(resolver_config, resolver_opts).unwrap();
    let response = resolver.lookup_ip(domain);

    match response {
        Ok(lookup_ip_result) => {
            for ip in lookup_ip_result.iter() {
                println!("Resolved IP: {}", ip);
            }
        }
        Err(e) => {
            eprintln!("Error resolving DNS: {:?}", e);
        }
    }

}