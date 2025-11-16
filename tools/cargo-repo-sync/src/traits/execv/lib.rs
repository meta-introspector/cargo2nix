use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use std::process::{Command, Output};
use std::ffi::OsStr;
use std::os::unix::prelude::ExitStatusExt;

// --- The Execv Trait ---
pub trait Execv {
    fn execv<S: AsRef<OsStr>>(&self, program: S, args: &[S]) -> Result<Output>;
}

pub struct ReportExecv<E: Execv> {
    inner: E,
}

impl<E: Execv> ReportExecv<E> {
    pub fn new(inner: E) -> Self {
        ReportExecv { inner }
    }
}

impl<E: Execv> Execv for ReportExecv<E> {
    fn execv<S: AsRef<OsStr>>(&self, program: S, args: &[S]) -> Result<Output> {
        let args_str: Vec<_> = args.iter().map(|s| s.as_ref().to_string_lossy()).collect();
        println!("[REPORT] Executing: {} {}", program.as_ref().to_string_lossy(), args_str.join(" "));
        let output = self.inner.execv(program, args)?;
        println!("[REPORT] Command finished with status: {:?}", output.status.code());
        if !output.stdout.is_empty() {
            println!("[REPORT] Stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("[REPORT] Stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(output)
    }
}

// --- 1. Dry Run Implementation ---
pub struct DryRunExecv<E: Execv> {
    inner: E,
}

impl<E: Execv> DryRunExecv<E> {
    pub fn new(inner: E) -> Self {
        DryRunExecv { inner }
    }
}

impl<E: Execv> Execv for DryRunExecv<E> {
    fn execv<S: AsRef<OsStr>>(&self, program: S, args: &[S]) -> Result<Output> {
        let args_str: Vec<_> = args.iter().map(|s| s.as_ref().to_string_lossy()).collect();
        println!("[DRY RUN] Would execute: {} {}", program.as_ref().to_string_lossy(), args_str.join(" "));
        // Return a successful, empty output without calling the inner executor
        Ok(Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    }
}

// --- 2. JSON Capture Implementation ---
pub struct JsonCaptureExecv<E: Execv> {
    inner: E,
    pub commands: std::sync::Mutex<Vec<CapturedCommand>>,
}

impl<E: Execv> JsonCaptureExecv<E> {
    pub fn new(inner: E) -> Self {
        JsonCaptureExecv {
            inner,
            commands: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl<E: Execv> Execv for JsonCaptureExecv<E> {
    fn execv<S: AsRef<OsStr>>(&self, program: S, args: &[S]) -> Result<Output> {
        let output = self.inner.execv(program.as_ref(), args)?;

        let captured_command = CapturedCommand {
            program: program.as_ref().to_string_lossy().to_string(),
            args: args.iter().map(|s| s.as_ref().to_string_lossy().to_string()).collect(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status.code(),
        };
        self.commands.lock().unwrap().push(captured_command);

        Ok(output)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapturedCommand {
    program: String,
    args: Vec<String>,
    stdout: String,
    stderr: String,
    status: Option<i32>,
}

// --- 3. System Command Implementation ---
pub struct SystemExecv;

impl Execv for SystemExecv {
    fn execv<S: AsRef<OsStr>>(&self, program: S, args: &[S]) -> Result<Output> {
        Command::new(program.as_ref())
            .args(args.iter().map(|s| s.as_ref()))
            .output()
            .with_context(|| format!("Failed to execute command: {:?}", program.as_ref()))
    }
}

// --- 4. Pure Rust Implementation ---
// sorry, not implemented yet
