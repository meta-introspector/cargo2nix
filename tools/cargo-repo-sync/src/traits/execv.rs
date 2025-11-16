use anyhow::{Context, Result};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::{Arc, Mutex},
};
use serde::{Serialize, Deserialize};
use std::os::unix::process::ExitStatusExt;

pub trait Execv: Send + Sync {
    fn execv(&self, program: &PathBuf, args: &[&str], current_dir: Option<&PathBuf>) -> Result<Output>;
}

pub struct SystemExecv;

impl Execv for SystemExecv {
    fn execv(&self, program: &PathBuf, args: &[&str], current_dir: Option<&PathBuf>) -> Result<Output> {
        let mut command = Command::new(program);
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        command.output().context(format!("Failed to execute command: {:?}", program))
    }
}

pub struct DryRunExecv {
    inner: Arc<dyn Execv>,
}

impl DryRunExecv {
    pub fn new(inner: Arc<dyn Execv>) -> Self {
        DryRunExecv { inner }
    }
}

impl Execv for DryRunExecv {
    fn execv(&self, program: &PathBuf, args: &[&str], current_dir: Option<&PathBuf>) -> Result<Output> {
        println!("[DRY RUN] Executing: {:?} {:?}", program, args);
        Ok(Output {
            stdout: b"Dry run output\n".to_vec(),
            stderr: vec![],
            status: std::process::ExitStatus::from_raw(0), // Success
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapturedCommand {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub current_dir: Option<PathBuf>,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub struct JsonCaptureExecv {
    inner: Arc<dyn Execv>,
    pub commands: Mutex<Vec<CapturedCommand>>,
}

impl JsonCaptureExecv {
    pub fn new(inner: Arc<dyn Execv>) -> Self {
        JsonCaptureExecv {
            inner,
            commands: Mutex::new(Vec::new()),
        }
    }
}

impl Execv for JsonCaptureExecv {
    fn execv(&self, program: &PathBuf, args: &[&str], current_dir: Option<&PathBuf>) -> Result<Output> {
        let output = self.inner.execv(program, args, current_dir)?;

        let captured_command = CapturedCommand {
            program: program.clone(),
            args: args.iter().map(|&s| s.to_string()).collect(),
            current_dir: current_dir.cloned(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        };

        self.commands.lock().unwrap().push(captured_command);
        Ok(output)
    }
}

pub struct ReportExecv {
    inner: Arc<dyn Execv>,
}

impl ReportExecv {
    pub fn new(inner: Arc<dyn Execv>) -> Self {
        ReportExecv { inner }
    }
}

impl Execv for ReportExecv {
    fn execv(&self, program: &PathBuf, args: &[&str], current_dir: Option<&PathBuf>) -> Result<Output> {
        println!("[REPORT] Executing: {:?} {:?}", program, args);
        let output = self.inner.execv(program, args, current_dir)?;
        println!("[REPORT] Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("[REPORT] Stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(output)
    }
}
