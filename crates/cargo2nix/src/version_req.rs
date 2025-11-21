use std::path::PathBuf;

use anyhow::{Context, Result};
use semver::{Version, VersionReq};

use crate::version_attributes;

pub fn version_req(path: &PathBuf) -> Result<(VersionReq, Version)> {
    let version = version_attributes::read_version_attribute(path)?;
    let req = format!(">={}.{}", version.major, version.minor);
    VersionReq::parse(&req)
        .context(format!("parse {} found in {}", req, path.display()))
        .map_err(anyhow::Error::from)
        .map(|req| (req, version))
}