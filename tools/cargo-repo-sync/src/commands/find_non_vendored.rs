use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};

enum InputReader {
    File(BufReader<fs::File>),
    Stdin(BufReader<tokio::io::Stdin>),
}

impl InputReader {
    async fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        match self {
            InputReader::File(reader) => reader.read_line(buf).await.map_err(anyhow::Error::new),
            InputReader::Stdin(reader) => reader.read_line(buf).await.map_err(anyhow::Error::new),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to the cargo tree output file (e.g., tree.txt). If not provided, reads from stdin.
    #[arg(long)]
    input_file: Option<PathBuf>,
    /// Path to the project root directory.
    #[arg(long)]
    project_root: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut reader = if let Some(input_file) = args.input_file {
        InputReader::File(BufReader::new(fs::File::open(&input_file).await?))
    } else {
        InputReader::Stdin(BufReader::new(tokio::io::stdin()))
    };

    let re = Regex::new(r"^(?P<name>[a-zA-Z0-9_-]+)\s+.*?\s+\((?P<path>[^)]+)\)$")?;

    println!("Identifying non-vendored modules...");

    let project_root_str = args
        .project_root
        .to_str()
        .context("Project root is not valid UTF-8")?
        .to_string();
    let submodules_prefix = format!("{}/submodules/", project_root_str);
    let vendor_prefix = format!("{}/vendor/", project_root_str);

    let mut line_buffer = String::new(); // Buffer to read lines into
    loop {
        line_buffer.clear(); // Clear the buffer for the new line
        let bytes_read = reader.read_line(&mut line_buffer).await?;

        if bytes_read == 0 {
            // EOF
            break;
        }

        let line = line_buffer.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(captures) = re.captures(line) {
            let name = captures.name("name").unwrap().as_str();
            let path_str = captures.name("path").unwrap().as_str();

            eprintln!("DEBUG: Name: '{}'", name);
            eprintln!("DEBUG: Path: '{}'", path_str);
            eprintln!("DEBUG: Submodules Prefix: '{}'", submodules_prefix);
            eprintln!("DEBUG: Vendor Prefix: '{}'", vendor_prefix);
            eprintln!(
                "DEBUG: path_str starts with submodules_prefix: {}",
                path_str.starts_with(&submodules_prefix)
            );
            eprintln!(
                "DEBUG: path_str starts with vendor_prefix: {}",
                path_str.starts_with(&vendor_prefix)
            );

            if !(path_str.starts_with(&submodules_prefix) || path_str.starts_with(&vendor_prefix)) {
                eprintln!("DEBUG: Printing non-vendored module: '{}'", name);
                println!("{}", name); // Output only the module name for non-vendored modules
            } else {
                eprintln!("DEBUG: NOT printing vendored module: '{}'", name);
            }
        } else {
            eprintln!("DEBUG: Regex did NOT match line: '{}'", line); // Keep this for now
        }
    }

    println!("Finished identifying non-vendored modules.");
    Ok(())
}
