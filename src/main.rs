mod cli;
mod resolver;

use cli::Args;
use resolver::resolve;

fn main() {
    let args = Args::parse_args();
    let domain_name = args.domain_name.unwrap();

    resolve(&domain_name);
}
