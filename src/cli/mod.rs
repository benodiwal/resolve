use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(required=true)]
    pub domain_name: Option<String>,

    #[clap(short = 's', long="dns-server", required=false)]
    pub dns_server: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}