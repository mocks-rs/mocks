mod console;
mod error;
mod server;
mod storage;

use crate::console::{print_error, print_init_aborted, print_init_success, print_startup_info};
use crate::error::MocksError;
use crate::server::Server;
use crate::storage::Storage;
use clap::Parser;
use std::net::{IpAddr, SocketAddr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Start the mock api server
    Run(RunArgs),
    /// Initialize a new storage file
    Init(InitArgs),
}

#[derive(clap::Args, Debug)]
struct RunArgs {
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

#[derive(clap::Args, Debug)]
struct InitArgs {
    /// Path of json file to create (default: storage.json)
    #[arg(default_value = "storage.json")]
    file: String,

    /// Create empty structure instead of sample data
    #[arg(short = 'E', long)]
    empty: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for NO_COLOR environment variable once at startup
    if std::env::var("NO_COLOR").is_ok() {
        colored::control::set_override(false);
    }

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Run(args) => {
            let socket_addr = match parse_socket_addr(&args.host, args.port) {
                Ok(addr) => addr,
                Err(e) => {
                    print_error(&e);
                    std::process::exit(1);
                }
            };

            let url = format!("http://{}:{}", &args.host, args.port);
            let overwrite = !args.no_overwrite;

            let storage = match Storage::new(&args.file, overwrite) {
                Ok(s) => s,
                Err(e) => {
                    print_error(&e);
                    std::process::exit(1);
                }
            };

            print_startup_info(&url, &args.file, overwrite);

            Server::startup(socket_addr, storage).await
        }
        Commands::Init(args) => {
            let result = Storage::init_file(&args.file, args.empty);
            match &result {
                Ok(()) => print_init_success(&args.file),
                Err(MocksError::Aborted) => {
                    print_init_aborted();
                    return Ok(());
                }
                Err(_) => {
                    // Other errors will be handled by the main error handling below
                }
            }
            result
        }
    };

    if let Err(e) = result {
        print_error(&e);
        std::process::exit(1);
    }

    Ok(())
}

fn parse_socket_addr(host: &str, port: u16) -> Result<SocketAddr, MocksError> {
    let ip_addr = if host == "localhost" {
        "127.0.0.1"
    } else {
        host
    };

    ip_addr
        .parse::<IpAddr>()
        .map(|ip| SocketAddr::from((ip, port)))
        .map_err(|e| MocksError::InvalidArgs(e.to_string()))
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(clap::builder::styling::AnsiColor::Blue.on_default().bold())
        .usage(clap::builder::styling::AnsiColor::Green.on_default().bold())
        .literal(clap::builder::styling::AnsiColor::BrightCyan.on_default())
        .placeholder(clap::builder::styling::AnsiColor::BrightWhite.on_default())
        .error(clap::builder::styling::AnsiColor::Red.on_default().bold())
        .valid(clap::builder::styling::AnsiColor::Green.on_default().bold())
        .invalid(clap::builder::styling::AnsiColor::Red.on_default().bold())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_with_localhost() {
        let result = parse_socket_addr("localhost", 3000).unwrap();
        assert_eq!(result.ip().to_string(), "127.0.0.1");
        assert_eq!(result.port(), 3000);
    }

    #[test]
    fn test_init_with_ip_address() {
        let result = parse_socket_addr("192.168.1.1", 8080).unwrap();
        assert_eq!(result.ip().to_string(), "192.168.1.1");
        assert_eq!(result.port(), 8080);
    }

    #[test]
    fn test_init_with_invalid_host() {
        let result = parse_socket_addr("invalid.host", 3000);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_styles() {
        let _ = get_styles();
    }
}
