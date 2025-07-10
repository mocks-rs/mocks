mod error;
mod server;
mod storage;

use crate::error::MocksError;
use crate::server::Server;
use crate::storage::Storage;
use clap::Parser;
use colored::*;
use std::net::{IpAddr, SocketAddr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles = get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Start the mock server
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
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Run(args) => {
            let socket_addr = match init(&args.host, args.port) {
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
        Commands::Init(args) => Storage::init_file(&args.file, args.empty),
    };

    if let Err(e) = result {
        print_error(&e);
        std::process::exit(1);
    }

    Ok(())
}

fn init(host: &str, port: u16) -> Result<SocketAddr, MocksError> {
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

fn print_startup_info(url: &str, file: &str, overwrite: bool) {
    // Check for NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        colored::control::set_override(false);
    }

    println!();
    println!("{}", "======================================".cyan());
    println!("{}", "mocks server started!".green().bold());
    println!("{}", "======================================".cyan());
    println!("{}", "Press CTRL-C to stop".yellow());
    println!();

    println!("{}", "Server Information:".blue().bold());
    println!("   {}: {}", "URL".bright_white(), url.bright_cyan());
    println!("   {}: {}", "Storage".bright_white(), file.bright_cyan());
    println!(
        "   {}: {}",
        "Overwrite".bright_white(),
        if overwrite { "YES".green() } else { "NO".red() }
    );
    println!();
}

fn get_styles() -> clap::builder::Styles {
    // Check for NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        return clap::builder::Styles::plain();
    }

    clap::builder::Styles::styled()
        .header(clap::builder::styling::AnsiColor::Blue.on_default().bold())
        .usage(clap::builder::styling::AnsiColor::Green.on_default().bold())
        .literal(clap::builder::styling::AnsiColor::BrightCyan.on_default())
        .placeholder(clap::builder::styling::AnsiColor::BrightWhite.on_default())
        .error(clap::builder::styling::AnsiColor::Red.on_default().bold())
        .valid(clap::builder::styling::AnsiColor::Green.on_default().bold())
        .invalid(clap::builder::styling::AnsiColor::Red.on_default().bold())
}

fn print_error(error: &MocksError) {
    // Check for NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        colored::control::set_override(false);
    }

    eprintln!("{}: {}", "Error".red().bold(), error.to_string().red());

    // Print additional context for some error types
    match error {
        MocksError::FailedReadFile(_) => {
            eprintln!(
                "{}: Check if the file exists and is readable",
                "Hint".bright_yellow()
            );
        }
        MocksError::FailedWriteFile(_) => {
            eprintln!(
                "{}: Check file permissions and disk space",
                "Hint".bright_yellow()
            );
        }
        MocksError::InvalidArgs(_) => {
            eprintln!(
                "{}: Run with --help to see usage information",
                "Hint".bright_yellow()
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_with_localhost() {
        let result = init("localhost", 3000).unwrap();
        assert_eq!(result.ip().to_string(), "127.0.0.1");
        assert_eq!(result.port(), 3000);
    }

    #[test]
    fn test_init_with_ip_address() {
        let result = init("192.168.1.1", 8080).unwrap();
        assert_eq!(result.ip().to_string(), "192.168.1.1");
        assert_eq!(result.port(), 8080);
    }

    #[test]
    fn test_init_with_invalid_host() {
        let result = init("invalid.host", 3000);
        assert!(result.is_err());
    }
}
