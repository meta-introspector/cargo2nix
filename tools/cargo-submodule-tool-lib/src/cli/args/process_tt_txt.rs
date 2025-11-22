#[cfg(feature = "clap_enabled")]
use clap::Parser;
use std::path::PathBuf;

#[cfg(feature = "clap_enabled")]
#[derive(Parser, Debug)]
pub struct ProcessTtTxtArgs {
    /// Path to the tt.txt file to be processed.
    #[arg(long, default_value = "tt.txt")]
    pub tt_txt_path: PathBuf,

    /// Path to the submodules directory.
    #[arg(long, default_value = "submodules")]
    pub submodules_dir: PathBuf,
}

#[cfg(not(feature = "clap_enabled"))]
#[derive(Debug)]
pub struct ProcessTtTxtArgs {
    pub tt_txt_path: PathBuf,
    pub submodules_dir: PathBuf,
}