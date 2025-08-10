use crate::error::MocksError;
use colored::*;

fn print_separator() {
    println!("{}", "======================================".cyan());
}

fn print_blank() {
    println!();
}

fn print_banner<T: std::fmt::Display>(text: T) {
    print_separator();
    println!("{text}");
    print_separator();
}

fn print_heading(text: &str) {
    println!("{}", text.blue().bold());
}

fn print_kv_with_indent(label: &str, value: impl std::fmt::Display) {
    println!("   {}: {}", label.bright_white(), value);
}

fn print_info(text: &str) {
    println!("{}", text.yellow());
}

pub fn print_startup_info(url: &str, file: &str, overwrite: bool) {
    print_banner("mocks server started!".green().bold());
    print_info("Press CTRL-C to stop");
    print_blank();

    print_heading("Server Information:");
    print_kv_with_indent("URL", url.bright_cyan());
    print_kv_with_indent("Storage", file.bright_cyan());
    print_kv_with_indent(
        "Overwrite",
        if overwrite { "YES".green() } else { "NO".red() },
    );
    print_blank();
}

pub fn print_init_success(file_path: &str) {
    print_banner("mocks initialized!".green().bold());
    println!("{} {}", "Created:".bright_white(), file_path.bright_cyan());
    print_blank();
}

pub fn print_init_aborted() {
    print_banner("mocks init aborted!".red().bold());
    println!("{}", "Aborted.".yellow());
    print_blank();
}

pub fn print_error(error: &MocksError) {
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
    fn test_print_separator() {
        print_separator();
    }

    #[test]
    fn test_print_blank() {
        print_blank();
    }

    #[test]
    fn test_print_banner() {
        print_banner("Test Banner");
    }

    #[test]
    fn test_print_heading() {
        print_heading("Test Heading");
    }

    #[test]
    fn test_print_kv_with_indent() {
        print_kv_with_indent("Label", "Value");
    }

    #[test]
    fn test_print_info() {
        print_info("Test Info");
    }

    #[test]
    fn test_print_startup_info() {
        let url = "http://localhost:3000";
        let file = "storage.json";
        let overwrite = true;
        print_startup_info(url, file, overwrite);
    }

    #[test]
    fn test_print_init_success() {
        let file_path = "storage.json";
        print_init_success(file_path);
    }

    #[test]
    fn test_print_init_aborted() {
        print_init_aborted();
    }

    #[test]
    fn test_print_error() {
        let error = MocksError::InvalidArgs("Invalid argument".to_string());
        print_error(&error);

        let error = MocksError::FailedReadFile("Failed to read file".to_string());
        print_error(&error);

        let error = MocksError::FailedWriteFile("Failed to write file".to_string());
        print_error(&error);

        let error = MocksError::Exception("Exception".to_string());
        print_error(&error);
    }
}
