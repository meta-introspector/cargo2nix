use anyhow::Result;
use std::path::{Path, PathBuf};

// Dummy types mimicking cargo_metadata's essential structures
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageId {
    pub repr: String,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub source: Option<String>,
    pub req: String,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub id: PackageId,
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_members: Vec<PackageId>,
    pub workspace_root: PathBuf,
}

pub trait CargoMetadataProvider: Send + Sync {
    fn provide_metadata(&self, project_root: &Path) -> Result<Metadata>;
}

// --- RealCargoMetadataProvider ---
#[cfg(feature = "real_cargo_metadata")]
pub struct RealCargoMetadataProvider;

#[cfg(feature = "real_cargo_metadata")]
impl CargoMetadataProvider for RealCargoMetadataProvider {
    fn provide_metadata(&self, project_root: &Path) -> Result<Metadata> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .current_dir(project_root)
            .exec()?;

        // Convert real cargo_metadata types to our dummy types
        let packages: Vec<Package> = metadata.packages.into_iter().map(|p| Package {
            id: PackageId { repr: p.id.repr },
            name: p.name,
            version: p.version.to_string(),
            manifest_path: p.manifest_path,
            dependencies: p.dependencies.into_iter().map(|d| Dependency {
                name: d.name,
                source: d.source.map(|s| s.repr),
                req: d.req.to_string(),
            }).collect(),
        }).collect();

        let workspace_members: Vec<PackageId> = metadata.workspace_members.into_iter().map(|id| PackageId { repr: id.repr }).collect();

        Ok(Metadata {
            packages,
            workspace_members,
            workspace_root: metadata.workspace_root,
        })
    }
}

// --- DummyCargoMetadataProvider ---
#[cfg(not(feature = "real_cargo_metadata"))]
pub struct DummyCargoMetadataProvider;

#[cfg(not(feature = "real_cargo_metadata"))]
impl CargoMetadataProvider for DummyCargoMetadataProvider {
    fn provide_metadata(&self, _project_root: &Path) -> Result<Metadata> {
        // Return dummy metadata for testing or when cargo_metadata is not available
        Ok(Metadata {
            packages: vec![
                Package {
                    id: PackageId { repr: "dummy-package-id-1".to_string() },
                    name: "dummy-package-1".to_string(),
                    version: "0.1.0".to_string(),
                    manifest_path: PathBuf::from("/dummy/path/dummy-package-1/Cargo.toml"),
                    dependencies: vec![],
                },
            ],
            workspace_members: vec![PackageId { repr: "dummy-package-id-1".to_string() }],
            workspace_root: PathBuf::from("/dummy/path"),
        })
    }
}
