use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ProcessTtTxtArgs {
    /// Path to the tt.txt file to be processed.
    #[arg(long, default_value = "tt.txt")]
    pub tt_txt_path: PathBuf,

    /// Path to the submodules directory.
    #[arg(long, default_value = "submodules")]
    pub submodules_dir: PathBuf,
}
