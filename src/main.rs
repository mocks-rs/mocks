mod error;
mod server;
mod storage;

use crate::error::MocksError;
use crate::server::Server;
use crate::storage::Storage;
use clap::Parser;
use std::net::{IpAddr, SocketAddr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of json file for data storage
    file: String,

    /// Host
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    /// Port
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// No overwrite save to json file
    #[arg(long, default_value_t = false)]
    no_overwrite: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match init(&args.host, args.port) {
        Ok(socket_addr) => {
            println!("`mocks` started");
            println!("Press CTRL-C to stop");

            let url = format!("http://{}:{}", &args.host, args.port);

            println!();
            println!("Index:");
            println!("{}", &url);

            println!();
            println!("Storage files:");
            println!("{}", args.file);

            println!();
            println!("Overwrite:");
            println!("{}", if !args.no_overwrite { "YES" } else { "NO" });

            match Storage::new(&args.file, !args.no_overwrite) {
                Ok(s) => {
                    // Run mock api server
                    match Server::startup(socket_addr, &url, s).await {
                        Ok(()) => {
                            println!();
                        }
                        Err(e) => {
                            println_err(&e);
                        }
                    }
                }
                Err(e) => {
                    println_err(&e);
                }
            }
        }
        Err(e) => {
            println_err(&e);
        }
    }
}

fn init(host: &str, port: u16) -> Result<SocketAddr, MocksError> {
    let ip_addr = if host == "localhost" {
        "127.0.0.1"
    } else {
        host
    };

    match ip_addr.parse::<IpAddr>() {
        Ok(ip_addr) => Ok(SocketAddr::from((ip_addr, port))),
        Err(e) => Err(MocksError::InvalidArgs(e.to_string())),
    }
}

fn println_err(e: &MocksError) {
    println!();
    println!("ERROR: {}", e);
}
