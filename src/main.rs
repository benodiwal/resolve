mod cli;

use cli::Args;


fn main() {
    let args = Args::parse_args();
    //let domain_name_raw = args.domain_name.unwrap();
    //let dns_server_raw = args.dns_server.unwrap();

    //let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    //let mut response_as_bytes: [u8; 512] = [0; 512];
    //
    println!("{}", args.domain_name.unwrap());
}
