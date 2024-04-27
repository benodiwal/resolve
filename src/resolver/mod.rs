use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::ResolverOpts;
use crate::utils::{resolver_configuration, DEFAULT_DNS_SERVER};

pub fn resolve(domain: &str, dns_server: Option<&str>) {
    let resolver_config = match dns_server {
        Some(ip_str) => {
            resolver_configuration(ip_str, false)
        }
        None => {
            resolver_configuration(DEFAULT_DNS_SERVER, true)
        }
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