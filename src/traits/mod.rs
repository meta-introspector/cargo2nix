use anyhow::Result;
use cargo::core::{Package, PackageId, Workspace};
use cargo::core::compiler::{CompileKind, RustcTargetData};
use cargo::core::resolver::{Resolve, CliFeatures, HasDevUnits, ForceAllTargets};
use cargo::ops::WorkspaceResolve; // Add WorkspaceResolve
use std::collections::{BTreeMap, HashSet};

// Re-export for convenience
pub use crate::template::{Optionality, ResolvedDependency, ResolvedPackage};

pub trait CargoResolver {
    fn resolve_with_all_features<'a>(
        &self,
        ws: &Workspace<'a>,
        target_data: &mut RustcTargetData<'a>,
        requested_kinds: &[CompileKind],
    ) -> Result<WorkspaceResolve<'a>>;

    fn resolve_with_no_features<'a>(
        &self,
        ws: &Workspace<'a>,
        target_data: &mut RustcTargetData<'a>,
        requested_kinds: &[CompileKind],
    ) -> Result<WorkspaceResolve<'a>>;

    fn mark_required(
        &self,
        resolved_no_features: &Resolve,
        rpkgs_by_id: &mut BTreeMap<PackageId, ResolvedPackage>,
    ) -> Result<()>;

    fn mark_feature_activations<'a, 'gctx>(
        &self,
        root_pkg: &'a Package,
        ws: &Workspace<'gctx>,
        resolved_no_features: &Resolve,
        rpkgs_by_id: &mut BTreeMap<PackageId, ResolvedPackage<'a>>,
        target_data: &mut RustcTargetData<'gctx>,
        compile_kinds: &[CompileKind],
    ) -> Result<()>;
}

pub struct DefaultCargoResolver;

impl CargoResolver for DefaultCargoResolver {
    fn resolve_with_all_features<'a>(
        &self,
        ws: &Workspace<'a>,
        target_data: &mut RustcTargetData<'a>,
        requested_kinds: &[CompileKind],
    ) -> Result<WorkspaceResolve<'a>> {
        let specs: Vec<cargo::core::PackageIdSpec> = ws.members().map(|p| p.package_id().to_spec()).collect();
        let force_all = ForceAllTargets::Yes; // Define force_all here
        cargo::ops::resolve_ws_with_opts(
            ws,
            target_data,
            requested_kinds,
            &CliFeatures::new_all(true),
            &specs,
            HasDevUnits::Yes,
            force_all,
            false,
        )
    }

    fn resolve_with_no_features<'a>(
        &self,
        ws: &Workspace<'a>,
        target_data: &mut RustcTargetData<'a>,
        requested_kinds: &[CompileKind],
    ) -> Result<WorkspaceResolve<'a>> {
        let specs: Vec<cargo::core::PackageIdSpec> = ws.members().map(|p| p.package_id().to_spec()).collect();
        let force_all = ForceAllTargets::Yes; // Define force_all here
        let no_features = CliFeatures::from_command_line(
            &[],   // no features
            false, // don't use all features
            false, // don't use default features
        )?;
        cargo::ops::resolve_ws_with_opts(
            ws,
            target_data,
            requested_kinds,
            &no_features,
            &specs,
            HasDevUnits::Yes,
            force_all,
            false,
        )
    }

    fn mark_required(
        &self,
        resolved_no_features: &Resolve,
        rpkgs_by_id: &mut BTreeMap<PackageId, ResolvedPackage>,
    ) -> Result<()> {
        // Dependencies that are activated, even when no features are activated, must be required.
        for id in resolved_no_features.iter() {
            let rpkg = rpkgs_by_id.get_mut(&id).unwrap();
            for feature in resolved_no_features.features(id).iter() {
                // unwrap doesn't fail because it's from resolved_all_features
                *(rpkg.features.get_mut(feature.as_str()).unwrap()) = Optionality::Required;
            }

            for (dep_id, _) in resolved_no_features.deps(id) {
                for dep in rpkg.iter_deps_with_id_mut(dep_id) {
                    dep.optionality = Optionality::Required;
                }
            }
        }
        Ok(())
    }

    fn mark_feature_activations<'a, 'gctx>(
        &self,
        root_pkg: &'a Package,
        ws: &Workspace<'gctx>,
        resolved_no_features: &Resolve,
        rpkgs_by_id: &mut BTreeMap<PackageId, ResolvedPackage<'a>>,
        target_data: &mut RustcTargetData<'gctx>,
        compile_kinds: &[CompileKind],
    ) -> Result<()> {
        let root_pkg_name = root_pkg.name().as_str();
        let root_pkg_features = root_pkg.summary().features();

        let spec = root_pkg.package_id().to_spec();

        for feature in root_pkg_features.keys() {
            // resolve ws with just the target feature activated
            let just_this_feature = CliFeatures::from_command_line(
                &[feature.to_string()], // just the active feature
                false,                  // don't use all features
                false,                  // don't use default features
            )?;

            let just_feature_ws = cargo::ops::resolve_ws_with_opts(
                ws,
                target_data,
                compile_kinds,
                &just_this_feature,
                &[spec.clone()],
                HasDevUnits::Yes,
                ForceAllTargets::Yes,
                false,
            )?;

            for rpkg in rpkgs_by_id.values_mut() {
                let deps_no_features: HashSet<_> = resolved_no_features
                    .deps(rpkg.pkg.package_id())
                    .map(|(dep, _)| dep)
                    .collect();

                let deps_just_feature: HashSet<_> = just_feature_ws
                    .targeted_resolve
                    .deps(rpkg.pkg.package_id())
                    .map(|(dep, _)| dep)
                    .collect();

                rpkg.deps
                    .iter_mut()
                    .map(|(_, rpkg)| rpkg)
                    .filter(|rpkg| {
                        !deps_no_features.contains(&rpkg.pkg.package_id())
                            && deps_just_feature.contains(&rpkg.pkg.package_id())
                    })
                    .for_each(|rpkg| rpkg.optionality.activated_by((root_pkg_name, feature)));

                let features_no_features: HashSet<_> = resolved_no_features
                    .features(rpkg.pkg.package_id())
                    .iter()
                    .map(|feature| feature.to_string())
                    .collect();

                let features_just_feature: HashSet<_> = just_feature_ws
                    .targeted_resolve
                    .features(rpkg.pkg.package_id())
                    .iter()
                    .map(|feature| feature.to_string())
                    .collect();

                rpkg.features
                    .iter_mut()
                    .filter(|(f, _)| {
                        !features_no_features.contains(&f.to_string())
                            && features_just_feature.contains(&f.to_string())
                    })
                    .for_each(|(_f, optionality)| optionality.activated_by((root_pkg_name, feature)));
            }
        }
        Ok(())
    }
}
