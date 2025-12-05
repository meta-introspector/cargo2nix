use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use cargo::core::resolver::Resolve;
use cargo::core::{GitReference, Package, PackageId, SourceId, dependency::DepKind};
use cargo_platform::Platform;
use serde::Serialize;

use crate::expr::BoolExpr;
use crate::manifest::TomlProfile;
use crate::platform;

type FeatureStr<'a> = &'a str;
type PackageName<'a> = &'a str;
type RootFeature<'a> = (PackageName<'a>, FeatureStr<'a>);

fn to_features<'a>(features: &BTreeMap<FeatureStr<'a>, Optionality<'a>>) -> Vec<Feature> {
    features
        .iter()
        .map(
            |(name, optionality)| match optionality.to_expr("rootFeatures'").simplify() {
                BoolExpr::True => Feature {
                    name: name.to_string(),
                    activated_by: None,
                },
                expr => Feature {
                    name: name.to_string(),
                    activated_by: Some(expr.to_nix().to_string()),
                },
            },
        )
        .collect()
}

#[derive(Debug, Serialize)]
pub struct BuildPlan {
    pub cargo2nix_version: String,
    pub cargo_lock_hash: String,
    pub root_features: Vec<String>,
    pub profiles: BTreeMap<String, String>,
    pub workspace_members: Vec<Member>,
    pub crates: Vec<Crate>,
}

impl BuildPlan {
    pub fn from_items(
        cargo_lock_hash: String,
        root_pkgs: Vec<&'_ Package>,
        profiles: TomlProfile,
        rpkgs_by_id: BTreeMap<PackageId, ResolvedPackage<'_>>,
        cwd: &Path,
    ) -> Result<Self> {
        let root_features = root_pkgs
            .iter()
            .map(|pkg| format!("{}/default", pkg.name()))
            .collect();

        let profiles = profiles
            .into_iter()
            .map(|(name, profile)| (name, toml::to_string(&profile).unwrap()))
            .map(|(name, toml)| (name, toml.replace("${", "\\${").escape_debug().to_string()))
            .collect();

        let workspace_members = root_pkgs
            .into_iter()
            .map(|pkg| Member {
                name: pkg.name().to_string(),
                version: pkg.version().to_string(),
            })
            .collect();

        let crates = rpkgs_by_id
            .into_iter()
            .map(|(pkg_id, resolved_pkg)| {
                let (deps, dev_deps, build_deps) = to_dependencies(&resolved_pkg);
                Ok(Crate {
                    name: pkg_id.name().to_string(),
                    version: pkg_id.version().to_string(),
                    registry: to_registry_string(pkg_id.source_id()),
                    source: to_source(&resolved_pkg, cwd)?,
                    features: to_features(&resolved_pkg.features),
                    dependencies: deps,
                    dev_dependencies: dev_deps,
                    build_dependencies: build_deps,
                })
            })
            .collect::<Result<_>>()?;

        Ok(BuildPlan {
            cargo2nix_version: env!("CARGO_PKG_VERSION").to_string(),
            cargo_lock_hash,
            root_features,
            profiles,
            workspace_members,
            crates,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Member {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub registry: String,
    pub source: Source,
    pub features: Vec<Feature>,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
    pub build_dependencies: Vec<Dependency>,
}

#[derive(Debug, Serialize)]
pub enum Source {
    CratesIo {
        sha256: String,
    },
    Git {
        url: String,
        rev: String,
        branch: Option<String>,
    },
    Local {
        path: PathBuf,
    },
    Registry {
        index: String,
        sha256: String,
    },
}

#[derive(Debug, Serialize)]
pub struct Feature {
    pub name: String,
    pub activated_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Dependency {
    pub name: String,
    pub extern_name: String,
    pub version: String,
    pub registry: String,
    pub cfg_condition: Option<String>,
    pub is_proc_macro: bool,
}

fn to_registry_string(src_id: SourceId) -> String {
    if src_id.is_path() {
        "unknown".to_string()
    } else if src_id.is_git() {
        format!("git+{}", src_id.url())
    } else {
        src_id.as_url().to_string()
    }
}

fn to_source(pkg: &ResolvedPackage<'_>, cwd: &Path) -> Result<Source> {
    let id = pkg.pkg.package_id();

    let source = if id.source_id().is_registry() {
        Source::CratesIo {
            sha256: pkg
                .checksum
                .as_ref()
                .map(|c| c.to_string())
                .ok_or(anyhow!("checksum is required for crates.io package {}", id))?,
        }
    } else if id.source_id().is_git() {
        let branch = if let Some(GitReference::Branch(branch)) = id.source_id().git_reference() {
            Some(branch.to_owned())
        } else {
            None
        };
        Source::Git {
            url: id.source_id().url().to_string(),
            rev: id
                .source_id()
                .precise_git_fragment()
                .map(|p| p.to_string())
                .ok_or(anyhow!("precise ref not found for git package {}", id))?,
            branch,
        }
    } else if id.source_id().is_path() {
        Source::Local {
            path: pathdiff::diff_paths(Path::new(id.source_id().url().path()), cwd)
                .map(|p| {
                    if p.to_string_lossy().len() == 0 {
                        p.join(".") // map degenerate empty path to "." for tera logic
                    } else {
                        p
                    }
                })
                .ok_or(anyhow!("path is not absolute for local package {}", id))?,
        }
    } else if id.source_id().is_registry() {
        Source::Registry {
            index: id.source_id().url().to_string(),
            sha256: pkg.checksum.as_ref().map(|c| c.to_string()).ok_or(anyhow!(
                "checksum is required for alternate registry package {}",
                id
            ))?,
        }
    } else {
        return Err(anyhow!("unsupported source for {}", id));
    };

    Ok(source)
}

fn to_dependencies(
    pkg: &ResolvedPackage<'_>,
) -> (Vec<Dependency>, Vec<Dependency>, Vec<Dependency>) {
    let mut dependencies = Vec::new();
    let mut dev_dependencies = Vec::new();
    let mut build_dependencies = Vec::new();

    for ((pkg_id, kind), dep) in &pkg.deps {
        let platforms = match dep.platforms {
            None => BoolExpr::True,
            Some(ref platforms) => BoolExpr::ors(
                platforms
                    .iter()
                    .map(|p| platform::to_expr(p, "hostPlatform")),
            ),
        };

        let cfg_condition = match dep
            .optionality
            .to_expr("rootFeatures'")
            .and(platforms)
            .simplify()
        {
            BoolExpr::True => None,
            expr => Some(expr.to_nix().to_string()),
        };

        let dep = Dependency {
            name: pkg_id.name().to_string(),
            extern_name: dep.extern_name.to_string(),
            version: pkg_id.version().to_string(),
            registry: to_registry_string(pkg_id.source_id()),
            cfg_condition,
            is_proc_macro: is_proc_macro(&dep.pkg),
        };

        match kind {
            DepKind::Normal => dependencies.push(dep),
            DepKind::Development => dev_dependencies.push(dep),
            DepKind::Build => build_dependencies.push(dep),
        }
    }

    (dependencies, dev_dependencies, build_dependencies)
}

#[derive(Debug)]
pub struct ResolvedPackage<'a> {
    pub pkg: &'a Package,
    pub deps: BTreeMap<(PackageId, DepKind), ResolvedDependency<'a>>,
    pub features: BTreeMap<FeatureStr<'a>, Optionality<'a>>,
    pub checksum: Option<&'a str>,
}

impl<'a> ResolvedPackage<'a> {
    pub fn new(
        pkg: &'a Package,
        pkgs_by_id: &HashMap<PackageId, &'a Package>,
        resolve: &'a Resolve,
    ) -> Result<Self> {
        let mut deps = BTreeMap::new();
        resolve
            .deps(pkg.package_id())
            .filter_map(|(dep_id, deps)| {
                let dep_pkg = pkgs_by_id[&dep_id];
                let extern_name = resolve
                    .extern_crate_name_and_dep_name(
                        pkg.package_id(),
                        dep_id,
                        dep_pkg.targets().iter().find(|t| t.is_lib())?,
                    )
                    .ok()?
                    .0
                    .to_string();

                Some(
                    deps.iter()
                        .map(move |dep| (dep_id, dep, dep_pkg, extern_name.clone())),
                )
            })
            .flatten()
            .for_each(|(dep_id, dep, dep_pkg, extern_name)| {
                let rdep = deps
                    .entry((dep_id, dep.kind()))
                    .or_insert(ResolvedDependency {
                        extern_name,
                        pkg: dep_pkg,
                        optionality: Optionality::default(),
                        platforms: Some(BTreeSet::new()),
                    });

                match (dep.platform(), rdep.platforms.as_mut()) {
                    (Some(platform), Some(platforms)) => {
                        platforms.insert(platform);
                    }
                    (None, _) => rdep.platforms = None,
                    _ => {}
                }
            });

        let features = resolve
            .features(pkg.package_id())
            .iter()
            .map(|feature| (feature.as_str(), Optionality::default()))
            .collect();

        let checksum = resolve
            .checksums()
            .get(&pkg.package_id())
            .and_then(|opt| opt.as_ref().map(|s| s.as_str()));

        Ok(Self {
            pkg,
            deps,
            features,
            checksum,
        })
    }

    pub fn iter_deps_with_id_mut(
        &mut self,
        id: PackageId,
    ) -> impl Iterator<Item = &mut ResolvedDependency<'a>> {
        self.deps
            .range_mut((id, DepKind::Normal)..=(id, DepKind::Build))
            .map(|(_, dep)| dep)
    }
}

#[derive(Debug)]
pub struct ResolvedDependency<'a> {
    pub extern_name: String,
    pub pkg: &'a Package,
    pub optionality: Optionality<'a>,
    pub platforms: Option<BTreeSet<&'a Platform>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Optionality<'a> {
    Required,
    Optional {
        activated_by_features: BTreeSet<RootFeature<'a>>,
    },
}

impl<'a> Default for Optionality<'a> {
    fn default() -> Self {
        Optionality::Optional {
            activated_by_features: Default::default(),
        }
    }
}

impl<'a> Optionality<'a> {
    pub fn activated_by(&mut self, (root_pkg_name, feature): RootFeature<'a>) {
        if let Optionality::Optional {
            activated_by_features,
        } = self
        {
            activated_by_features.insert((root_pkg_name, feature));
        }
    }

    pub fn to_expr(&self, root_features_var: &str) -> BoolExpr {
        match self {
            Optionality::Required => BoolExpr::True,
            Optionality::Optional {
                activated_by_features,
            } => BoolExpr::ors(activated_by_features.iter().map(|root_feature| {
                BoolExpr::Single(format!(
                    "{} ? {:?}",
                    root_features_var,
                    display_root_feature(*root_feature)
                ))
            })),
        }
    }
}

pub fn display_root_feature((pkg_name, feature): RootFeature) -> String {
    format!("{}/{}", pkg_name, feature)
}

pub fn simplify_optionality<'a, 'b: 'a>(
    rpkgs: impl IntoIterator<Item = &'a mut ResolvedPackage<'b>>,
) {
    for rpkg in rpkgs.into_iter() {
        // Dev dependencies can't be optional.
        rpkg.deps
            .iter_mut()
            .filter(|((_, kind), _)| *kind == DepKind::Development)
            .for_each(|(_, d)| d.optionality = Optionality::Required);

        // If a package's dependencies or features are activated identically to
        // the features it is activated by, reduce that dependency or feature
        // logic to required
        // TODO
        // For each package, for each feature & dependency, if optionality is
        // identical between package and feature / dependency, set feature /
        // dependency optionality to required
    }
}

pub fn is_proc_macro(pkg: &Package) -> bool {
    use cargo::core::TargetKind;
    use cargo::core::compiler::CrateType;
    pkg.targets()
        .iter()
        .filter_map(|t| match t.kind() {
            TargetKind::Lib(kinds) => Some(kinds.iter()),
            _ => None,
        })
        .flatten()
        .any(|k| *k == CrateType::ProcMacro)
}
