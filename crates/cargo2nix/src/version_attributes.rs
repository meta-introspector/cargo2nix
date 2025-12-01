use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

use anyhow::{Context, Result, anyhow};
use semver::Version;

use crate::version_constants;

pub fn read_version_attribute(path: &PathBuf) -> Result<Version> {
    let file = fs::File::open(path).context(format!("Couldn't open file {}", path.display()))?;
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .find(|line| line.trim_start().starts_with(version_constants::VERSION_ATTRIBUTE_NAME))
        .and_then(|s| {
            if let Some(i) = s.find('"') {
                if let Some(j) = s.rfind('"') {
                    return Version::parse(&s[i + 1..j]).ok();
                }
            }
            None
        })
        .ok_or_else(|| {
            anyhow!(
                "Could not find a valid '{}' in '{}'. This might happen if the file is empty or malformed. If you are generating Cargo.nix for the first time, you can safely delete the existing file or use the --overwrite flag.",
                version_constants::VERSION_ATTRIBUTE_NAME,
                path.display()
            )
        })
}
