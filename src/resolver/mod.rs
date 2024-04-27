use trust_dns_resolver::Resolver;

pub fn resolve(domain: &str) {
    let resolver = Resolver::default().unwrap();
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