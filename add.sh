#!/usr/bin/env bash
git add 	CargoRepSyncCargo.toml	build_and_report_errors.sh
rm cargo_build_output.txt 	cargo_repo_sync_deps.txt	cargo_tree_output.txt	cargoerror	commitmessage.txt	gemini_commit_message.txt
git add 	crates/	docs/sop/enforce_pure_functional_code.md	flake-oci.nix	flake-ssh.nix	generate_keyword_greps.sh 	generate_specific_keyword_greps.sh
rm gitstatus
#	index/
	log.txt
git add 	oci/	rust-toolchain.toml
rm 	submodules/build.log
rm 	submodules/compiler-builtins.txt
git add	submodules/dummy_crate/
rm 	submodules/erro.txt
rm	submodules/files.txt
rm	submodules/lint.xt
rm	submodules/log.txt
rm	submodules/steps.txt
git add 	submodules/update.sh
git add 	tasks/
git add 	tools/cargo-repo-sync-lib/	tools/cargo-submodule-tool/	tools/cargo-workspace-from-tree/	tools/secret-manager-rs/ 	workspaces/Cargo.lock 	workspaces/Cargo.toml	workspaces/anyhow_test/ 	workspaces/arraydeque_test/ 	workspaces/dummy_crate_test/
rm  	x86.txt
