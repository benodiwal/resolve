mod cli;
mod resolver;
mod utils;

use cli::Args;
use resolver::resolve;

fn main() {
    let args = Args::parse_args();
    let domain_name = args.domain_name.unwrap();
    let dns_server = args.dns_server.as_deref();

    resolve(&domain_name, dns_server);
}