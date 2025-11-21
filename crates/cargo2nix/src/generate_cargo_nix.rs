use hex; // Add this import
use std::{
    fs,
    io::Read, // Add Read trait
    path::PathBuf,
};

use anyhow::{Context, Result}; // Add Context trait
use cargo::{
    core::{
        compiler::{CompileKind, RustcTargetData},
        Workspace,
    },
    ops::{resolve_with_previous},
    util::important_paths::find_root_manifest_for_wd,
};
use cargo::core::resolver::features::{CliFeatures, HasDevUnits};
use sha2::{Digest, Sha256};
use tera::Tera;

use crate::manifest;
use crate::template::{BuildPlan, ResolvedPackage};
use crate::traits::CargoResolver;

pub fn generate_cargo_nix(workspace_directory: &PathBuf, locked: bool) -> Result<String> {
    let config = {
        let mut config = cargo::util::GlobalContext::default()?;
        config.configure(0, false, None, false, locked, false, &None, &[], &[])?;
        config
    };

    let root_manifest_path = find_root_manifest_for_wd(workspace_directory)?;
    let ws = Workspace::new(&root_manifest_path, &config)?;

    if !locked {
        let mut registry = ws.package_registry()?;
        let mut resolve = resolve_with_previous(
            &mut registry,
            &ws,
            &CliFeatures::new_all(true),
            HasDevUnits::Yes,
            None,
            None,
            &[],
            true,
        )?;
        cargo::ops::write_pkg_lockfile(&ws, &mut resolve)?;
    }

    // To get a list of all packages with all features and dependencies that
    // might be enabled present, we first resolve with all features turned on
    let requested_kinds = CompileKind::from_requested_targets(ws.gctx(), &[])?;
    let mut target_data = RustcTargetData::new(&ws, &requested_kinds)?;

    let cargo_resolver = crate::traits::DefaultCargoResolver; // Instantiate the resolver

    // Resolve entire workspace with all features.
    let resolved_all_opts = cargo_resolver.resolve_with_all_features(&ws, &mut target_data, &requested_kinds)?;
    let _resolved_all = &resolved_all_opts.targeted_resolve;

    let pkgs_by_id = resolved_all_opts
        .pkg_set
        .get_many(resolved_all_opts.pkg_set.package_ids())?
        .iter()
        .map(|pkg| (pkg.package_id(), *pkg))
        .collect();

    let mut rpkgs_by_id = resolved_all_opts
        .pkg_set
        .get_many(resolved_all_opts.pkg_set.package_ids())?
        .iter()
        .map(|pkg| {
            ResolvedPackage::new(pkg, &pkgs_by_id, &resolved_all_opts.targeted_resolve) // Use resolved_all_opts.targeted_resolve
                .map(|res| (pkg.package_id(), res))
        })
        .collect::<Result<_>>()?;

    // Resolve with just packages but no features turned on.  We can compare
    // with this `Resolve` to detect if turning on a feature made a
    // dependency or feature appear
    let resolved_no_features_opts = cargo_resolver.resolve_with_no_features(&ws, &mut target_data, &requested_kinds)?;
    let resolved_no_features = &resolved_no_features_opts.targeted_resolve;

    let root_pkgs: Vec<_> = ws.members().collect();

    // using the resolved_no_features Resolve, if a package or feature is
    // turned on, then it doesn't depend on any top-level features. While it
    // could depend on which workspace package is built, we always have to
    // assume that any workspace package could be built and build at least
    // that much.
    cargo_resolver.mark_required(&resolved_no_features, &mut rpkgs_by_id)?;

    for pkg in root_pkgs.iter() {
        cargo_resolver.mark_feature_activations(
            pkg,
            &ws,
            &resolved_no_features,
            &mut rpkgs_by_id,
            &mut target_data,
            &requested_kinds,
        )?;
    }

    // Certain optionality cases are redundant, such as including an optional
    // dependency always activates its feature.  These cases are reduced to
    // Optionality::Required
    crate::template::simplify_optionality(rpkgs_by_id.values_mut());

    let root_manifest = fs::read_to_string(&root_manifest_path)?;
    let profiles = manifest::extract_profiles(&root_manifest);

    let cargo_lock_path = root_manifest_path.clone().with_file_name("Cargo.lock");
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(&cargo_lock_path).context(format!("Does the Cargo.lock file exist at {}?", cargo_lock_path.display()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    let cargo_lock_hash: String = hex::encode(hasher.finalize());
    let plan = BuildPlan::from_items(
        cargo_lock_hash,
        root_pkgs,
        profiles,
        rpkgs_by_id,
        workspace_directory,
    )?;
    let mut tera = Tera::default();
    tera.add_raw_template(
        "Cargo.nix.tera",
        include_str!("../../../templates/Cargo.nix.tera"),
    )?;
    let context = tera::Context::from_serialize(plan)?;
    let rendered = tera.render("Cargo.nix.tera", &context)?;

    Ok(rendered)
}
