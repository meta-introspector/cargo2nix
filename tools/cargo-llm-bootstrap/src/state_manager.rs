use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::error::AppError;
use crate::results::CompilationResult; // Import CompilationResult

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum FileStatus {
    Pending,
    Completed, // Successfully compiled, result in output_dir
    Failed,
    Done,      // Successfully compiled, result moved to done_dir
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    #[serde(default = "default_file_status")]
    pub status: FileStatus,
    pub last_attempt_timestamp: Option<DateTime<Utc>>,
}

fn default_file_status() -> FileStatus {
    FileStatus::Pending
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileIndex {
    pub files: HashMap<PathBuf, FileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub rust_src_path_hash: String,
    pub index_file_paths: Vec<PathBuf>,
    pub last_saved_timestamp: DateTime<Utc>,
    pub output_dir: PathBuf, // Directory where compilation results are initially saved
    pub done_dir: PathBuf,   // Directory for successfully processed files
    pub cache: HashMap<PathBuf, CompilationResult>, // New cache field
}

impl State {


    pub fn load(main_state_path: &Path, output_dir: PathBuf, done_dir: PathBuf) -> Result<Self, AppError> {
        if main_state_path.exists() {
            println!("Loading main state from: {:?}", main_state_path.display());
            let content = fs::read_to_string(main_state_path).map_err(AppError::Io)?;
            match serde_json::from_str::<State>(&content) {
                Ok(mut loaded_state) => {
                    // Ensure output_dir and done_dir are updated if they changed
                    loaded_state.output_dir = output_dir;
                    loaded_state.done_dir = done_dir;
                    Ok(loaded_state)
                },
                Err(e) => {
                    println!("Warning: Failed to parse main state file at {:?}: {}. Returning default state.", main_state_path, e);
                    Ok(State {
                        rust_src_path_hash: String::new(),
                        index_file_paths: Vec::new(),
                        last_saved_timestamp: Utc::now(),
                        output_dir,
                        done_dir,
                        cache: HashMap::new(), // Initialize cache
                    })
                }
            }
        } else {
            println!("Main state file not found at {:?}. Returning default state.", main_state_path);
            Ok(State {
                rust_src_path_hash: String::new(),
                index_file_paths: Vec::new(),
                last_saved_timestamp: Utc::now(),
                output_dir,
                done_dir,
                cache: HashMap::new(), // Initialize cache
            })
        }
    }

    pub fn save(&self, main_state_path: &Path) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(self).map_err(AppError::Serde)?;
        fs::write(main_state_path, content).map_err(AppError::Io)?;
        Ok(())
    }

    pub fn update_file_status(&mut self, file_path: &Path, status: FileStatus) -> Result<(), AppError> {
        for index_path in &self.index_file_paths {
            let content = fs::read_to_string(index_path).map_err(AppError::Io)?;
            match serde_json::from_str::<FileIndex>(&content) {
                Ok(mut file_index) => {
                    if let Some(entry) = file_index.files.get_mut(file_path) {
                        entry.status = status;
                        entry.last_attempt_timestamp = Some(Utc::now());
                        self.last_saved_timestamp = Utc::now(); // Update main state timestamp

                        let updated_content = serde_json::to_string_pretty(&file_index).map_err(AppError::Serde)?;
                        fs::write(index_path, updated_content).map_err(AppError::Io)?;
                        return Ok(());
                    }
                },
                Err(e) => {
                    println!("Warning: Failed to parse file index at {:?}: {}. Skipping this index file for update.", index_path, e);
                }
            }
        }
        Err(AppError::Custom(format!("File not found in state: {:?}", file_path)))
    }

    pub fn get_pending_files(&self) -> Result<Vec<PathBuf>, AppError> {
        let mut pending_files = Vec::new();
        for index_path in &self.index_file_paths {
            let content = fs::read_to_string(index_path).map_err(AppError::Io)?;
            match serde_json::from_str::<FileIndex>(&content) {
                Ok(file_index) => {
                    pending_files.extend(
                        file_index.files
                            .values()
                            .filter(|entry| entry.status == FileStatus::Pending || entry.status == FileStatus::Failed)
                            .map(|entry| entry.path.clone())
                    );
                },
                Err(e) => {
                    println!("Warning: Failed to parse file index at {:?}: {}. Skipping this index file.", index_path, e);
                }
            }
        }
        Ok(pending_files)
    }

    pub fn get_last_failed_file(&self) -> Result<Option<PathBuf>, AppError> {
        let mut failed_files_with_timestamp: Vec<(PathBuf, DateTime<Utc>)> = Vec::new();

        for index_path in &self.index_file_paths {
            let content = fs::read_to_string(index_path).map_err(AppError::Io)?;
            let file_index: FileIndex = serde_json::from_str(&content).map_err(AppError::Serde)?;

            for entry in file_index.files.values() {
                if entry.status == FileStatus::Failed {
                    if let Some(timestamp) = entry.last_attempt_timestamp {
                        failed_files_with_timestamp.push((entry.path.clone(), timestamp));
                    }
                }
            }
        }

        failed_files_with_timestamp.sort_by_key(|(_, timestamp)| *timestamp);

        Ok(failed_files_with_timestamp.last().map(|(path, _)| path.clone()))
    }
}
