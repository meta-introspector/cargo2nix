use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::error::AppError;
// Removed: use crate::hasher::hash_directory; // Import the hash_directory function

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
    pub status: FileStatus,
    pub last_attempt_timestamp: Option<DateTime<Utc>>,
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
}

impl State {


    pub fn load(main_state_path: &Path, output_dir: PathBuf, done_dir: PathBuf) -> Result<Self, AppError> {
        if main_state_path.exists() {
            println!("Loading main state from: {:?}", main_state_path.display());
            let content = fs::read_to_string(main_state_path).map_err(AppError::Io)?;
            let mut loaded_state: State = serde_json::from_str(&content).map_err(AppError::Serde)?;

            // Ensure output_dir and done_dir are updated if they changed
            loaded_state.output_dir = output_dir;
            loaded_state.done_dir = done_dir;

            Ok(loaded_state)
        } else {
            Err(AppError::Custom(format!("Main state file not found: {:?}", main_state_path)))
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
            let mut file_index: FileIndex = serde_json::from_str(&content).map_err(AppError::Serde)?;

            if let Some(entry) = file_index.files.get_mut(file_path) {
                entry.status = status;
                entry.last_attempt_timestamp = Some(Utc::now());
                self.last_saved_timestamp = Utc::now(); // Update main state timestamp

                let updated_content = serde_json::to_string_pretty(&file_index).map_err(AppError::Serde)?;
                fs::write(index_path, updated_content).map_err(AppError::Io)?;
                return Ok(());
            }
        }
        Err(AppError::Custom(format!("File not found in state: {:?}", file_path)))
    }

    pub fn get_pending_files(&self) -> Result<Vec<PathBuf>, AppError> {
        let mut pending_files = Vec::new();
        for index_path in &self.index_file_paths {
            let content = fs::read_to_string(index_path).map_err(AppError::Io)?;
            let file_index: FileIndex = serde_json::from_str(&content).map_err(AppError::Serde)?;

            pending_files.extend(
                file_index.files
                    .values()
                    .filter(|entry| entry.status == FileStatus::Pending || entry.status == FileStatus::Failed)
                    .map(|entry| entry.path.clone())
            );
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
