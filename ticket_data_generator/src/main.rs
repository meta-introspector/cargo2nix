use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader, BufRead, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketData {
    pub ticket_id: String,
    pub target_name: String,
    pub error_reason: String, // e.g., "compiler-message"
    pub summary_message: String,
    pub occurrences: Vec<TicketOccurrence>,
    pub search_commands: Vec<String>, // New field for search commands
    pub referenced_files: Vec<ReferencedFile>,
    pub project_documentation: Vec<ProjectDocument>,
    pub macro_expanded_code_paths: Vec<String>, // Placeholder
    pub external_documentation_links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketOccurrence {
    pub rendered_message: String,
    pub file_path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub code_snippet: String, // The code around the error
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReferencedFile {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDocument {
    pub doc_path: String,
    pub content: String,
}

// Structs for deserializing the incoming cargo JSON messages
#[derive(Debug, Deserialize)]
pub struct CargoMessage {
    pub reason: String,
    pub package_id: Option<String>,
    pub manifest_path: Option<String>,
    pub target: Option<CargoTarget>,
    pub message: Option<CargoDiagnostic>,
}

#[derive(Debug, Deserialize)]
pub struct CargoTarget {
    pub kind: Vec<String>,
    pub crate_types: Vec<String>,
    pub name: String,
    pub src_path: String,
    pub edition: String,
    pub doc: bool,
    pub doctest: bool,
    pub test: bool,
}

#[derive(Debug, Deserialize)]
pub struct CargoDiagnostic {
    pub rendered: String,
    pub level: String,
    pub message: String,
    pub spans: Vec<CargoSpan>,
}

#[derive(Debug, Deserialize)]
pub struct CargoSpan {
    pub byte_end: usize,
    pub byte_start: usize,
    pub column_end: usize,
    pub column_start: usize,
    pub expansion: Option<CargoSpanExpansion>,
    pub file_name: String,
    pub is_primary: bool,
    pub label: Option<String>,
    pub line_end: usize,
    pub line_start: usize,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<String>,
    pub text: Vec<CargoSpanText>,
}

#[derive(Debug, Deserialize)]
pub struct CargoSpanExpansion {
    pub byte_end: usize,
    pub byte_start: usize,
    pub column_end: usize,
    pub column_start: usize,
    pub file_name: String,
    pub line_end: usize,
    pub line_start: usize,
    pub macro_decl_name: String,
    pub macro_decl_span: Option<Box<CargoSpan>>, // Box to avoid recursive type issues
    pub span: Box<CargoSpan>, // Box to avoid recursive type issues
}

#[derive(Debug, Deserialize)]
pub struct CargoSpanText {
    pub highlight_end: usize,
    pub highlight_start: usize,
    pub text: String,
}

// Helper to read file lines and extract a code snippet
fn read_file_lines_and_extract_snippet(file_path: &Path, line_start: usize, line_end: usize) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut snippet = String::new();
    let mut current_line = 1;
    let context_lines = 2; // Lines before and after for context

    for line_result in reader.lines() {
        let line = line_result?;
        if current_line >= line_start.saturating_sub(context_lines) && current_line <= line_end + context_lines {
            snippet.push_str(&line);
            snippet.push('\n');
        } else if current_line > line_end + context_lines {
            break; // Stop reading once past the relevant lines
        }
        current_line += 1;
    }
    Ok(snippet)
}

/// A simplified heuristic to resolve Rust 'use' paths to potential file paths.
/// This is highly imperfect and does not perform full module resolution.
fn resolve_use_path_to_file(
    use_path_str: &str,
    source_file_path: &Path,
    project_root: &Path,
) -> Option<PathBuf> {
    // Remove "::" at the end if present (e.g., "std::collections::" vs "std::collections")
    let use_path_str = use_path_str.trim_end_matches("::");
    let parts: Vec<&str> = use_path_str.split("::").collect();
    if parts.is_empty() {
        return None;
    }

    let mut potential_paths: Vec<PathBuf> = Vec::new();

    // Handle `crate::` and relative paths (`self::`, `super::`)
    if parts[0] == "crate" {
        let mut path = project_root.join("src");
        for part in parts.iter().skip(1) {
            path.push(part);
        }
        potential_paths.push(path.with_extension("rs"));
        potential_paths.push(path.join("mod.rs"));
    } else if parts[0] == "self" || parts[0] == "super" {
        // Resolve relative to the current source file's directory
        let mut current_dir = source_file_path.parent()?;
        if parts[0] == "super" {
            current_dir = current_dir.parent()?;
        }
        let mut path = current_dir.to_path_buf();
        for part in parts.iter().skip(1) {
            path.push(part);
        }
        potential_paths.push(path.with_extension("rs"));
        potential_paths.push(path.join("mod.rs"));
    } else {
        // Try resolving as a module within the same directory as the source file
        // or a sibling module within the current crate's src/ directory
        let source_file_dir = source_file_path.parent()?;
        let mut path = source_file_dir.to_path_buf();
        path.push(parts[0]);
        potential_paths.push(path.with_extension("rs"));
        potential_paths.push(path.join("mod.rs"));

        // Also try resolving relative to the project's src directory for top-level modules
        let mut path = project_root.join("src");
        path.push(parts[0]);
        potential_paths.push(path.with_extension("rs"));
        potential_paths.push(path.join("mod.rs"));
    }

    for p in potential_paths {
        if p.exists() && p.is_file() {
            return Some(p);
        }
    }

    None
}


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <build_json_path> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let build_json_path = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    if !build_json_path.exists() {
        eprintln!("Error: build_json_path '{}' does not exist.", build_json_path.display());
        std::process::exit(1);
    }

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    } else if !output_dir.is_dir() {
        eprintln!("Error: output_dir '{}' is not a directory.", output_dir.display());
        std::process::exit(1);
    }

    let file = File::open(&build_json_path)?;
    let reader = BufReader::new(file);

    let mut compiler_messages_by_target: HashMap<String, Vec<CargoMessage>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<CargoMessage>(&line) {
            Ok(msg) => {
                if msg.reason == "compiler-message" {
                    if let Some(target) = &msg.target {
                        compiler_messages_by_target
                            .entry(target.name.clone())
                            .or_default()
                            .push(msg);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse JSON line: {}. Error: {}", line, e);
            }
        }
    }

    let project_root = build_json_path.parent().unwrap_or(Path::new(".")); // Assume project root is parent of build.json
    let docs_dir = project_root.join("docs");

    for (target_name, messages) in compiler_messages_by_target {
        let mut ticket_occurrences: Vec<TicketOccurrence> = Vec::new();
        let mut referenced_files_map: HashMap<PathBuf, ReferencedFile> = HashMap::new();
        // Regex to capture the path in `use` statements.
        // This regex tries to capture the path part like `std::collections::HashMap` or `crate::module::Item`
        let use_regex = Regex::new(r"^\s*use\s+((?:[a-zA-Z_][a-zA-Z_0-9]*::)*[a-zA-Z_][a-zA-Z_0-9]*)(?:[^;]*)?;$").unwrap();
        let mut search_commands: Vec<String> = Vec::new();


        for msg in messages {
            if let Some(diagnostic) = msg.message {
                if let Some(span) = diagnostic.spans.iter().find(|s| s.is_primary) {
                    let file_path_str = &span.file_name;
                    let file_path = PathBuf::from(file_path_str);
                    let full_file_path = project_root.join(&file_path);

                    let code_snippet = match read_file_lines_and_extract_snippet(
                        &full_file_path,
                        span.line_start,
                        span.line_end,
                    ) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Warning: Could not read snippet from {}: {}", full_file_path.display(), e);
                            String::new()
                        }
                    };

                    ticket_occurrences.push(TicketOccurrence {
                        rendered_message: diagnostic.rendered.clone(), // Clone to use later
                        file_path: file_path_str.clone(),
                        line_start: span.line_start,
                        line_end: span.line_end,
                        code_snippet,
                    });

                    // Add the primary source file itself to referenced files
                    if full_file_path.exists() && full_file_path.is_file() {
                        if let Ok(content) = fs::read_to_string(&full_file_path) {
                            referenced_files_map.entry(full_file_path.clone()).or_insert(ReferencedFile {
                                file_path: file_path.display().to_string(), // Store path relative to project root
                                content,
                            });
                        }
                    }

                    // Generate search commands based on the diagnostic
                    // Command for the primary error location
                    search_commands.push(format!("rg --context 5 \"{}\" {}", regex::escape(&diagnostic.message), file_path.display()));

                    // If it's an unused import warning, generate specific search commands
                    if diagnostic.message.contains("unused import") {
                        if let Some(captures) = Regex::new(r"unused import: `([^`]+)`").unwrap().captures(&diagnostic.message) {
                            let unused_import_path = captures.get(1).unwrap().as_str();
                            search_commands.push(format!("rg --context 5 \"use {}\" {}", regex::escape(unused_import_path), file_path.display()));
                            search_commands.push(format!("rg \"{}\" {}", regex::escape(unused_import_path), file_path.display())); // Search for usage
                        }
                    }
                    // For unexpected `cfg` condition (from rustc_llvm example)
                    if diagnostic.message.contains("unexpected `cfg` condition name") {
                        if let Some(captures) = Regex::new(r"unexpected `cfg` condition name: `([^`]+)`").unwrap().captures(&diagnostic.message) {
                            let cfg_condition = captures.get(1).unwrap().as_str();
                            search_commands.push(format!("rg --context 5 \"{}\" {}", regex::escape(cfg_condition), file_path.display()));
                            search_commands.push(format!("rg --context 5 \"{}\" Cargo.toml", regex::escape(cfg_condition)));
                            search_commands.push(format!("rg --context 5 \"{}\" build.rs", regex::escape(cfg_condition)));
                        }
                    }

                    // Try to identify referenced files via `use` statements in the primary source file
                    if let Ok(content) = fs::read_to_string(&full_file_path) {
                        for line in content.lines() {
                            if let Some(captures) = use_regex.captures(line) {
                                let use_path_str = captures.get(1).unwrap().as_str();

                                if let Some(resolved_path) = resolve_use_path_to_file(use_path_str, &full_file_path, project_root) {
                                    if resolved_path.exists() && resolved_path.is_file() {
                                        if let Ok(ref_content) = fs::read_to_string(&resolved_path) {
                                            let relative_path = resolved_path.strip_prefix(project_root).unwrap_or(&resolved_path);
                                            referenced_files_map.entry(resolved_path.clone()).or_insert(ReferencedFile {
                                                file_path: relative_path.display().to_string(),
                                                content: ref_content,
                                            });
                                        }
                                    }
                                } else {
                                    eprintln!("Debug: Could not resolve use path '{}' from file '{}'", use_path_str, full_file_path.display());
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut project_documentation: Vec<ProjectDocument> = Vec::new();
        if docs_dir.is_dir() {
            for entry in fs::read_dir(&docs_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md" || ext == "txt") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if content.contains(&target_name) {
                            project_documentation.push(ProjectDocument {
                                doc_path: path.strip_prefix(project_root).unwrap_or(&path).display().to_string(),
                                content,
                            });
                        }
                    }
                }
            }
        }
        
        let referenced_files: Vec<ReferencedFile> = referenced_files_map.into_values().collect();

        // Calculate a simple hash for ticket_id
        let mut hasher = Sha256::new();
        hasher.update(&target_name);
        if let Some(first_occurrence) = ticket_occurrences.first() {
            hasher.update(&first_occurrence.rendered_message); // Use first message for part of hash
        }
        let ticket_id = hex::encode(hasher.finalize());

        let ticket_data = TicketData {
            ticket_id,
            target_name: target_name.clone(),
            error_reason: "compiler-message".to_string(),
            summary_message: format!("Compiler messages for target: {}", target_name),
            occurrences: ticket_occurrences,
            search_commands: search_commands.clone(), // Add generated search commands
            referenced_files,
            project_documentation,
            macro_expanded_code_paths: Vec::new(), // Placeholder
            external_documentation_links: vec![
                format!("https://docs.rs/{}/latest", target_name),
                "https://doc.rust-lang.org/book/".to_string(),
            ],
        };

        // Output the full ticket data as JSON
        let output_json_path = output_dir.join(format!("ticket_{}.json", ticket_data.ticket_id));
        let mut output_json_file = File::create(&output_json_path)?;
        serde_json::to_writer_pretty(&mut output_json_file, &ticket_data)?;
        println!("Generated ticket JSON file: {}", output_json_path.display());

        // Also generate a markdown file with search commands
        let output_md_path = output_dir.join(format!("task_{}.md", ticket_data.ticket_id));
        let mut output_md_file = File::create(&output_md_path)?;
        writeln!(&mut output_md_file, "# Task: Resolve {} (Ticket ID: {})", ticket_data.summary_message, ticket_data.ticket_id)?;
        writeln!(&mut output_md_file, "")?;
        if let Some(first_occ) = ticket_data.occurrences.first() {
            writeln!(&mut output_md_file, "**Primary Occurrence:**")?;
            writeln!(&mut output_md_file, "- File: `{}`", first_occ.file_path)?;
            writeln!(&mut output_md_file, "- Line: {}", first_occ.line_start)?;
            writeln!(&mut output_md_file, "- Message: {}", first_occ.rendered_message)?;
            writeln!(&mut output_md_file, "")?;
        }
        writeln!(&mut output_md_file, "## Investigation Commands")?;
        for cmd in &ticket_data.search_commands {
            writeln!(&mut output_md_file, "```bash\n{}
```", cmd)?;
        }
        writeln!(&mut output_md_file, "")?;
        
        println!("Generated task markdown file: {}", output_md_path.display());
    }
    
    Ok(())
}
