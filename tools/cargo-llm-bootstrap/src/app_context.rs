use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::config::CompilerConfig;
use crate::state_manager::State;
use crate::result_store::JsonResultStore;

pub struct AppContext {
    pub config: CompilerConfig,
    pub crate_name_to_root_map: HashMap<String, PathBuf>,
    pub layered_crates: HashMap<String, u32>,
    pub compiled_artifacts_map: HashMap<String, PathBuf>,
    pub state: Arc<Mutex<State>>,
    pub result_store: JsonResultStore,
    pub output_dir: PathBuf,
    pub done_dir: PathBuf,
}

impl AppContext {
    pub fn new(
        config: CompilerConfig,
        crate_name_to_root_map: HashMap<String, PathBuf>,
        layered_crates: HashMap<String, u32>,
        state: State,
        output_dir: PathBuf,
        done_dir: PathBuf,
    ) -> Self {
        Self {
            config,
            crate_name_to_root_map,
            layered_crates,
            compiled_artifacts_map: HashMap::new(),
            state: Arc::new(Mutex::new(state)),
            result_store: JsonResultStore::new(),
            output_dir,
            done_dir,
        }
    }

    pub fn get_max_layer(&self) -> u32 {
        self.layered_crates.values().max().cloned().unwrap_or(0)
    }
}

pub const STATE_FILE_NAME: &str = "main_state.json";
pub const DONE_DIR_NAME: &str = "done_results";
