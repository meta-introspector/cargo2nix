import toml
import sys

def process_cargo_toml(file_path):
    with open(file_path, 'r') as f:
        content = toml.load(f)

    # Dependencies to ensure are non-optional
    non_optional_deps = [
        "anyhow",
        "syn",
        "serde_json",
        "split-expanded-lib",
        "pipeline-traits",
        "prettyplease",
        "cargo_metadata",
        "walkdir",
        "toml",
        "lazy_static",
        "tokio",
        "once_cell",
        "quote",
        "proc-macro2",
        "regex",
        "glob",
        "chrono",
        "serde",
        "tempfile",
        "sha2",
        "base64",
        "indoc",
    ]

    # Dependencies that are explicitly optional (workspace or regular) and might have _enabled features
    explicitly_optional_deps = {
        "cargo-submodule-tool-lib": { "workspace": True, "optional": True },
        "toml_edit": { "version": "*", "optional": True },
        "clap": { "version": "*", "optional": True },
        "cargo-repo-sync-lib": { "workspace": True, "optional": True },
        "git2": { "version": "*", "optional": True },
        "hex": { "version": "*", "optional": True }, # Added hex here
        "sha1": { "version": "*", "optional": True },
        "pathdiff": { "version": "*", "optional": True },
        "git-wrapper-lib": { "workspace": True, "optional": True },
    }


    # Process [dependencies] section
    if 'dependencies' in content:
        for dep_name in non_optional_deps:
            if dep_name in content['dependencies']:
                dep_info = content['dependencies'][dep_name]
                if isinstance(dep_info, dict):
                    if 'optional' in dep_info:
                        del dep_info['optional']
                    if 'workspace' in dep_info and 'path' not in dep_info and 'version' not in dep_info:
                         dep_info['workspace'] = True # Ensure it's explicitly workspace = true if it was just { workspace = true, optional = true }
                content['dependencies'][dep_name] = dep_info # Update the content
            elif dep_name == "anyhow":
                content['dependencies'][dep_name] = "1.0"
            elif dep_name == "syn":
                content['dependencies'][dep_name] = { "version": "2.0", "features": ["full", "extra-traits"] }
            elif dep_name == "serde_json":
                content['dependencies'][dep_name] = "1.0"
            elif dep_name == "split-expanded-lib":
                content['dependencies'][dep_name] = { 'path': '../split-expanded-lib' }
            elif dep_name == "pipeline-traits":
                content['dependencies'][dep_name] = { 'path': '../pipeline-traits' }
            elif dep_name == "prettyplease":
                content['dependencies'][dep_name] = "0.2"
            elif dep_name == "cargo_metadata":
                content['dependencies'][dep_name] = { 'workspace': True }
            elif dep_name == "walkdir":
                content['dependencies'][dep_name] = "2"
            elif dep_name == "toml":
                content['dependencies'][dep_name] = "0.8.13"
            elif dep_name == "lazy_static":
                content['dependencies'][dep_name] = "1.4.0"
            elif dep_name == "tokio":
                content['dependencies'][dep_name] = { "version": "1", "features": ["full"] }
            elif dep_name == "once_cell":
                content['dependencies'][dep_name] = "1.19.0"
            elif dep_name == "quote":
                content['dependencies'][dep_name] = "1.0"
            elif dep_name == "proc-macro2":
                content['dependencies'][dep_name] = { "version": "1.0", "features": ["span-locations"] }
            elif dep_name == "regex":
                content['dependencies'][dep_name] = "1"
            elif dep_name == "glob":
                content['dependencies'][dep_name] = "0.3.0"
            elif dep_name == "chrono":
                content['dependencies'][dep_name] = { "version": "0.4", "features": ["serde"] }
            elif dep_name == "serde":
                content['dependencies'][dep_name] = { "version": "1.0", "features": ["derive"] }
            elif dep_name == "tempfile":
                content['dependencies'][dep_name] = "3.8"
            elif dep_name == "sha2":
                content['dependencies'][dep_name] = "0.10"
            elif dep_name == "base64":
                content['dependencies'][dep_name] = "0.21"
            elif dep_name == "indoc":
                content['dependencies'][dep_name] = "1.0"


        # Add missing explicitly optional dependencies
        for dep_name, dep_info in explicitly_optional_deps.items():
            if dep_name not in content['dependencies']:
                content['dependencies'][dep_name] = dep_info
            else: # Ensure existing optional dependencies have 'optional: true'
                existing_dep_info = content['dependencies'][dep_name]
                if isinstance(existing_dep_info, dict):
                    existing_dep_info['optional'] = True
                    content['dependencies'][dep_name] = existing_dep_info


        # Ensure ast-decoder is a path dependency and optional
        if "ast-decoder" in content['dependencies']:
            if isinstance(content['dependencies']['ast-decoder'], dict):
                content['dependencies']['ast-decoder']['path'] = '../ast-decoder'
                content['dependencies']['ast-decoder']['optional'] = True
            else:
                content['dependencies']['ast-decoder'] = { 'path': '../ast-decoder', 'optional': True }


    # Process [features] section
    if 'features' in content:
        all_feature_names = list(content['features'].keys()) # To avoid modifying dict during iteration
        for feature_name in all_feature_names:
            if not feature_name.endswith("_enabled"):
                continue

            dep_name_from_feature = feature_name.replace("_enabled", "").replace("-", "_") # Handle kebab-case to snake_case

            # Check if this feature's dependency is explicitly non-optional
            if dep_name_from_feature in non_optional_deps:
                if feature_name in content['features']: # Ensure it still exists
                    del content['features'][feature_name]
            
            # Check if this feature's dependency is an explicitly optional dependency
            elif dep_name_from_feature in explicitly_optional_deps:
                # Ensure the 'dep:' syntax is correctly used if the feature still exists
                if feature_name in content['features']:
                    current_feature_deps = content['features'][feature_name]
                    if not any(f"dep:{dep_name_from_feature}" == dep for dep in current_feature_deps): # Exact match "dep:X"
                        # If 'dep:X' is missing, add it
                        if isinstance(current_feature_deps, list):
                            if not current_feature_deps: # if list is empty
                                content['features'][feature_name] = [f"dep:{dep_name_from_feature}"]
                            else:
                                content['features'][feature_name].append(f"dep:{dep_name_from_feature}")
                        else: # if it's a string
                            content['features'][feature_name] = [f"dep:{dep_name_from_feature}"]
            
            else: # Remove feature if its dependency is not handled as non-optional or explicitly optional
                if feature_name in content['features']:
                    del content['features'][feature_name]


    with open(file_path, 'w') as f:
        toml.dump(content, f)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python process_cargo_toml.py <path_to_Cargo.toml>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    process_cargo_toml(file_path)