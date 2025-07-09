mod error;
mod server;
mod storage;

use crate::error::MocksError;
use crate::server::Server;
use crate::storage::Storage;
use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
async fn main() -> Result<(), MocksError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => {
            let socket_addr = init(&args.host, args.port)?;

            println!("`mocks` started");
            println!("Press CTRL-C to stop");

            let url = format!("http://{}:{}", &args.host, args.port);
            let overwrite = !args.no_overwrite;

            print_startup_info(&url, &args.file, overwrite);

            let storage = Storage::new(&args.file, overwrite)?;
            Server::startup(socket_addr, &url, storage).await?;
        }
        Commands::Init(args) => {
            init_storage_file(&args.file, args.empty)?;
        }
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

fn init_storage_file(file_path: &str, empty: bool) -> Result<(), MocksError> {
    let path = Path::new(file_path);
    
    if path.exists() {
        print!("File {} already exists. Overwrite? (y/N): ", file_path);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Aborted.");
            return Ok(());
        }
    }
    
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| MocksError::InvalidArgs(format!("Failed to create directory: {}", e)))?;
        }
    }
    
    let content = if empty {
        r#"{
  "posts": [],
  "profile": {}
}"#
    } else {
        r#"{
  "posts": [
    {
      "id": 1,
      "title": "Hello World",
      "content": "This is a sample post"
    }
  ],
  "profile": {
    "id": 1,
    "name": "Sample User"
  }
}"#
    };
    
    fs::write(path, content)
        .map_err(|e| MocksError::InvalidArgs(format!("Failed to write file: {}", e)))?;
    
    println!("Created: {}", file_path);
    Ok(())
}

fn print_startup_info(url: &str, file: &str, overwrite: bool) {
    println!("\nIndex:\n{url}");
    println!("\nStorage files:\n{file}");
    println!("\nOverwrite:\n{}", if overwrite { "YES" } else { "NO" });
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

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

    #[test]
    fn test_init_storage_file_creates_default_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();
        
        let result = init_storage_file(file_path_str, false);
        assert!(result.is_ok());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\""));
        assert!(content.contains("\"Hello World\""));
        assert!(content.contains("\"profile\""));
        assert!(content.contains("\"Sample User\""));
    }

    #[test]
    fn test_init_storage_file_creates_empty_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();
        
        let result = init_storage_file(file_path_str, true);
        assert!(result.is_ok());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\": []"));
        assert!(content.contains("\"profile\": {}"));
        assert!(!content.contains("Hello World"));
        assert!(!content.contains("Sample User"));
    }

    #[test]
    fn test_init_storage_file_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("sub").join("dir").join("test.json");
        let file_path_str = file_path.to_str().unwrap();
        
        let result = init_storage_file(file_path_str, false);
        assert!(result.is_ok());
        
        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"posts\""));
    }

    #[test]
    fn test_init_storage_file_with_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("existing.json");
        let file_path_str = file_path.to_str().unwrap();
        
        fs::write(&file_path, "existing content").unwrap();
        assert!(file_path.exists());
        
        let result = init_storage_file(file_path_str, false);
        assert!(result.is_ok());
    }
}
