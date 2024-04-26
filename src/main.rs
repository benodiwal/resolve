use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(required=true)]
    domain_name: Option<String>,

    #[clap(short = 's', long="dns-server", required=false, default_value = "1.1.1.1")]
    dns_server: Option<String>,
}

fn main() {
    let args = Args::parse();
    let domain_name_raw = args.domain_name.unwrap();
    let dns_server_raw = args.dns_server.unwrap();

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: [u8; 512] = [0; 512];
}
