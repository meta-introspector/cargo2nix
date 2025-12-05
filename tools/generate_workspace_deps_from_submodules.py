import re
import os
from collections import defaultdict

def generate_submodule_path_map(submodules_dir):
    """
    Scans the submodules directory and creates a mapping from crate names to their submodule paths.
    This mapping tries to be intelligent about common Rust submodule naming conventions.
    """
    submodule_path_map = {}
    
    # First pass: direct matches and common patterns
    for entry in os.listdir(submodules_dir):
        full_path = os.path.join(submodules_dir, entry)
        if os.path.isdir(full_path):
            # Direct match
            submodule_path_map[entry] = f"submodules/{entry}"
            
            # Handle -rs suffix convention
            if entry.endswith("-rs"):
                crate_name_without_suffix = entry[:-3]
                submodule_path_map[crate_name_without_suffix] = f"submodules/{entry}"
            
            # Handle nested crates within a submodule (e.g., cargo, time-rs, powerfmt, deranged)
            # This requires inspecting the submodule's Cargo.toml or having specific rules.
            # For now, let's add some specific heuristics based on the tt.txt content.
            if entry == "cargo":
                # cargo/credential/cargo-credential
                # cargo/crates/cargo-platform
                for sub_entry in os.listdir(full_path):
                    sub_full_path = os.path.join(full_path, sub_entry)
                    if os.path.isdir(sub_full_path):
                        if sub_entry == "credential":
                            for cred_entry in os.listdir(sub_full_path):
                                cred_full_path = os.path.join(sub_full_path, cred_entry)
                                if os.path.isdir(cred_full_path) and cred_entry.startswith("cargo-credential"):
                                    submodule_path_map[cred_entry] = f"submodules/cargo/credential/{cred_entry}"
                        elif sub_entry == "crates":
                            for crate_entry in os.listdir(sub_full_path):
                                crate_full_path = os.path.join(sub_full_path, crate_entry)
                                if os.path.isdir(crate_full_path) and crate_entry.startswith("cargo-"):
                                    submodule_path_map[crate_entry] = f"submodules/cargo/crates/{crate_entry}"
            elif entry == "time-rs":
                for time_entry in os.listdir(full_path):
                    time_full_path = os.path.join(full_path, time_entry)
                    if os.path.isdir(time_full_path) and time_entry.startswith("time"):
                        submodule_path_map[time_entry] = f"submodules/time-rs/{time_entry}"
            elif entry == "powerfmt":
                for powerfmt_entry in os.listdir(full_path):
                    powerfmt_full_path = os.path.join(full_path, powerfmt_entry)
                    if os.path.isdir(powerfmt_full_path) and powerfmt_entry.startswith("powerfmt"):
                        submodule_path_map[powerfmt_entry] = f"submodules/powerfmt/{powerfmt_entry}"
            elif entry == "deranged":
                for deranged_entry in os.listdir(full_path):
                    deranged_full_path = os.path.join(full_path, deranged_entry)
                    if os.path.isdir(deranged_full_path) and deranged_entry.startswith("deranged"):
                        submodule_path_map[deranged_entry] = f"submodules/deranged/{deranged_entry}"
            elif entry == "serde":
                for serde_entry in os.listdir(full_path):
                    serde_full_path = os.path.join(full_path, serde_entry)
                    if os.path.isdir(serde_full_path) and serde_entry.startswith("serde"):
                        submodule_path_map[serde_entry] = f"submodules/serde/{serde_entry}"
            elif entry == "clap":
                for clap_entry in os.listdir(full_path):
                    clap_full_path = os.path.join(full_path, clap_entry)
                    if os.path.isdir(clap_full_path) and clap_entry.startswith("clap"):
                        submodule_path_map[clap_entry] = f"submodules/clap/{clap_entry}"
            elif entry == "rust-url": # For "url" crate
                submodule_path_map["url"] = f"submodules/{entry}"
            elif entry == "lazy-static": # For "lazy_static" crate
                submodule_path_map["lazy_static"] = f"submodules/{entry}"
            elif entry == "sha1-smol": # For "sha1" crate
                submodule_path_map["sha1"] = f"submodules/{entry}"
            elif entry == "git2-rs": # For "git2" crate
                submodule_path_map["git2"] = f"submodules/{entry}"
            elif entry == "libgit2-sys": # For "libgit2-sys" crate
                submodule_path_map["libgit2-sys"] = f"submodules/{entry}"
            elif entry == "log": # For "log" crate
                submodule_path_map["log"] = f"submodules/{entry}"
            elif entry == "lru-rs": # For "lru" crate
                submodule_path_map["lru"] = f"submodules/{entry}"
            elif entry == "toml_edit": # For "toml_edit" crate
                submodule_path_map["toml_edit"] = f"submodules/{entry}"
            elif entry == "walkdir": # For "walkdir" crate
                submodule_path_map["walkdir"] = f"submodules/{entry}"
            elif entry == "regex": # For "regex" crate
                submodule_path_map["regex"] = f"submodules/{entry}"
            elif entry == "serde_json": # For "serde_json" crate
                submodule_path_map["serde_json"] = f"submodules/{entry}"
            elif entry == "pathdiff": # For "pathdiff" crate
                submodule_path_map["pathdiff"] = f"submodules/{entry}"
            elif entry == "md5": # For "md5" crate
                submodule_path_map["md5"] = f"submodules/{entry}"
            elif entry == "sha2": # For "sha2" crate
                submodule_path_map["sha2"] = f"submodules/{entry}"
            elif entry == "cargo_metadata": # For "cargo_metadata" crate
                submodule_path_map["cargo_metadata"] = f"submodules/{entry}"
            elif entry == "annotate-snippets-rs": # For "annotate-snippets" crate
                submodule_path_map["annotate-snippets"] = f"submodules/{entry}"
            elif entry == "anstyle": # For "anstyle" crate
                submodule_path_map["anstyle"] = f"submodules/{entry}"
                submodule_path_map["anstream"] = f"submodules/{entry}" # anstream is part of anstyle
            elif entry == "anyhow": # For "anyhow" crate
                submodule_path_map["anyhow"] = f"submodules/{entry}"
            elif entry == "rust-base64": # For "base64" crate
                submodule_path_map["base64"] = f"submodules/{entry}"
            elif entry == "blake3": # For "blake3" crate
                submodule_path_map["blake3"] = f"submodules/{entry}"
            elif entry == "build-rs": # For "build-rs" crate
                submodule_path_map["build-rs"] = f"submodules/{entry}"
            elif entry == "color-print": # For "color-print" crate
                submodule_path_map["color-print"] = f"submodules/{entry}"
            elif entry == "core-foundation-rs": # For "core-foundation" crate
                submodule_path_map["core-foundation"] = f"submodules/{entry}"
            elif entry == "crates-io": # For "crates-io" crate
                submodule_path_map["crates-io"] = f"submodules/cargo/crates/{entry}" # Specific path
            elif entry == "criterion": # For "criterion" crate
                submodule_path_map["criterion"] = f"submodules/{entry}"
            elif entry == "curl-rust": # For "curl" crate
                submodule_path_map["curl"] = f"submodules/{entry}"
            elif entry == "curl-sys": # For "curl-sys" crate
                submodule_path_map["curl-sys"] = f"submodules/{entry}"
            elif entry == "filetime": # For "filetime" crate
                submodule_path_map["filetime"] = f"submodules/{entry}"
            elif entry == "flate2-rs": # For "flate2" crate
                submodule_path_map["flate2"] = f"submodules/{entry}"
            elif entry == "gix": # For "gix" crate
                submodule_path_map["gix"] = f"submodules/{entry}"
            elif entry == "glob": # For "glob" crate
                submodule_path_map["glob"] = f"submodules/{entry}"
            elif entry == "handlebars-rust": # For "handlebars" crate
                submodule_path_map["handlebars"] = f"submodules/{entry}"
            elif entry == "hex": # For "hex" crate
                submodule_path_map["hex"] = f"submodules/{entry}"
            elif entry == "hmac": # For "hmac" crate
                submodule_path_map["hmac"] = f"submodules/{entry}"
            elif entry == "home": # For "home" crate
                submodule_path_map["home"] = f"submodules/{entry}"
            elif entry == "http-auth": # For "http-auth" crate
                submodule_path_map["http-auth"] = f"submodules/{entry}"
            elif entry == "ignore": # For "ignore" crate
                submodule_path_map["ignore"] = f"submodules/{entry}"
            elif entry == "im-rc": # For "im-rc" crate
                submodule_path_map["im-rc"] = f"submodules/{entry}"
            elif entry == "indexmap": # For "indexmap" crate
                submodule_path_map["indexmap"] = f"submodules/{entry}"
            elif entry == "itertools": # For "itertools" crate
                submodule_path_map["itertools"] = f"submodules/{entry}"
            elif entry == "jiff": # For "jiff" crate
                submodule_path_map["jiff"] = f"submodules/{entry}"
            elif entry == "jobserver-rs": # For "jobserver" crate
                submodule_path_map["jobserver"] = f"submodules/{entry}"
            elif entry == "libc": # For "libc" crate
                submodule_path_map["libc"] = f"submodules/{entry}"
            elif entry == "libloading": # For "libloading" crate
                submodule_path_map["libloading"] = f"submodules/{entry}"
            elif entry == "memchr": # For "memchr" crate
                submodule_path_map["memchr"] = f"submodules/{entry}"
            elif entry == "miow": # For "miow" crate
                submodule_path_map["miow"] = f"submodules/{entry}"
            elif entry == "opener": # For "opener" crate
                submodule_path_map["opener"] = f"submodules/{entry}"
            elif entry == "openssl": # For "openssl" crate
                submodule_path_map["openssl"] = f"submodules/{entry}"
            elif entry == "openssl-sys": # For "openssl-sys" crate
                submodule_path_map["openssl-sys"] = f"submodules/{entry}"
            elif entry == "os_info": # For "os_info" crate
                submodule_path_map["os_info"] = f"submodules/{entry}"
            elif entry == "pasetors": # For "pasetors" crate
                submodule_path_map["pasetors"] = f"submodules/{entry}"
            elif entry == "pathdiff": # For "pathdiff" crate
                submodule_path_map["pathdiff"] = f"submodules/{entry}"
            elif entry == "percent-encoding": # For "percent-encoding" crate
                submodule_path_map["percent-encoding"] = f"submodules/{entry}"
            elif entry == "pkg-config-rs": # For "pkg-config" crate
                submodule_path_map["pkg-config"] = f"submodules/{entry}"
            elif entry == "proptest": # For "proptest" crate
                submodule_path_map["proptest"] = f"submodules/{entry}"
            elif entry == "pulldown-cmark": # For "pulldown-cmark" crate
                submodule_path_map["pulldown-cmark"] = f"submodules/{entry}"
            elif entry == "rand": # For "rand" crate
                submodule_path_map["rand"] = f"submodules/{entry}"
            elif entry == "regex": # For "regex" crate
                submodule_path_map["regex"] = f"submodules/{entry}"
            elif entry == "rusqlite": # For "rusqlite" crate
                submodule_path_map["rusqlite"] = f"submodules/{entry}"
            elif entry == "rustc-hash": # For "rustc-hash" crate
                submodule_path_map["rustc-hash"] = f"submodules/{entry}"
            elif entry == "rustc-stable-hash": # For "rustc-stable-hash" crate
                submodule_path_map["rustc-stable-hash"] = f"submodules/{entry}"
            elif entry == "rustfix": # For "rustfix" crate
                submodule_path_map["rustfix"] = f"submodules/cargo/crates/{entry}" # Specific path
            elif entry == "same-file": # For "same-file" crate
                submodule_path_map["same-file"] = f"submodules/{entry}"
            elif entry == "schemars": # For "schemars" crate
                submodule_path_map["schemars"] = f"submodules/{entry}"
            elif entry == "security-framework": # For "security-framework" crate
                submodule_path_map["security-framework"] = f"submodules/{entry}"
            elif entry == "semver": # For "semver" crate
                submodule_path_map["semver"] = f"submodules/{entry}"
            elif entry == "serde": # For "serde" crate
                submodule_path_map["serde"] = f"submodules/{entry}"
            elif entry == "serde-untagged": # For "serde-untagged" crate
                submodule_path_map["serde-untagged"] = f"submodules/{entry}"
            elif entry == "serde-value": # For "serde-value" crate
                submodule_path_map["serde-value"] = f"submodules/{entry}"
            elif entry == "serde_core": # For "serde_core" crate
                submodule_path_map["serde_core"] = f"submodules/{entry}"
            elif entry == "serde_ignored": # For "serde_ignored" crate
                submodule_path_map["serde_ignored"] = f"submodules/{entry}"
            elif entry == "serde_json": # For "serde_json" crate
                submodule_path_map["serde_json"] = f"submodules/{entry}"
            elif entry == "sha1": # For "sha1" crate
                submodule_path_map["sha1"] = f"submodules/sha1-smol" # Specific path
            elif entry == "sha2": # For "sha2" crate
                submodule_path_map["sha2"] = f"submodules/{entry}"
            elif entry == "shell-escape": # For "shell-escape" crate
                submodule_path_map["shell-escape"] = f"submodules/{entry}"
            elif entry == "similar": # For "similar" crate
                submodule_path_map["similar"] = f"submodules/{entry}"
            elif entry == "snapbox": # For "snapbox" crate
                submodule_path_map["snapbox"] = f"submodules/{entry}"
            elif entry == "supports-hyperlinks": # For "supports-hyperlinks" crate
                submodule_path_map["supports-hyperlinks"] = f"submodules/{entry}"
            elif entry == "supports-unicode": # For "supports-unicode" crate
                submodule_path_map["supports-unicode"] = f"submodules/{entry}"
            elif entry == "tar": # For "tar" crate
                submodule_path_map["tar"] = f"submodules/tar-rs" # Specific path
            elif entry == "tempfile": # For "tempfile" crate
                submodule_path_map["tempfile"] = f"submodules/{entry}"
            elif entry == "thiserror": # For "thiserror" crate
                submodule_path_map["thiserror"] = f"submodules/{entry}"
            elif entry == "time": # For "time" crate
                submodule_path_map["time"] = f"submodules/time-rs/time" # Specific path
            elif entry == "toml": # For "toml" crate
                submodule_path_map["toml"] = f"submodules/{entry}"
            elif entry == "toml_edit": # For "toml_edit" crate
                submodule_path_map["toml_edit"] = f"submodules/{entry}"
            elif entry == "tracing": # For "tracing" crate
                submodule_path_map["tracing"] = f"submodules/{entry}"
            elif entry == "tracing-chrome": # For "tracing-chrome" crate
                submodule_path_map["tracing-chrome"] = f"submodules/{entry}"
            elif entry == "tracing-subscriber": # For "tracing-subscriber" crate
                submodule_path_map["tracing-subscriber"] = f"submodules/{entry}"
            elif entry == "unicase": # For "unicase" crate
                submodule_path_map["unicase"] = f"submodules/{entry}"
            elif entry == "unicode-ident": # For "unicode-ident" crate
                submodule_path_map["unicode-ident"] = f"submodules/{entry}"
            elif entry == "unicode-width": # For "unicode-width" crate
                submodule_path_map["unicode-width"] = f"submodules/{entry}"
            elif entry == "url": # For "url" crate
                submodule_path_map["url"] = f"submodules/rust-url" # Specific path
            elif entry == "varisat": # For "varisat" crate
                submodule_path_map["varisat"] = f"submodules/{entry}"
            elif entry == "walkdir": # For "walkdir" crate
                submodule_path_map["walkdir"] = f"submodules/{entry}"
            elif entry == "windows-sys": # For "windows-sys" crate
                submodule_path_map["windows-sys"] = f"submodules/{entry}"
            elif entry == "winnow": # For "winnow" crate
                submodule_path_map["winnow"] = f"submodules/{entry}"

    return submodule_path_map

def process_tt_txt_accurate(tt_txt_path, submodules_dir):
    output_lines = []
    submodule_map = generate_submodule_path_map(submodules_dir)

    with open(tt_txt_path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                output_lines.append("")
                continue

            # Regex to capture crate name and the rest of the dependency definition
            match = re.match(r'(\w[\w-]*)\s*=\s*(.*)', line)
            if not match:
                output_lines.append(line) # Keep lines that don't match the pattern
                continue

            crate_name = match.group(1)
            dep_definition = match.group(2)

            # Check if it's already a path dependency
            path_match = re.search(r'path\s*=\s*"(.*?)"', dep_definition)
            if path_match:
                # If it's already a path dependency, just ensure the path is correct
                # based on our submodule_map, or keep it if it's a custom path.
                if crate_name in submodule_map:
                    output_lines.append(f'{crate_name} = {{ path = "{submodule_map[crate_name]}" }}')
                else:
                    # If it's a path dependency but not in our submodule map, keep original
                    output_lines.append(line)
            else:
                # It's a version dependency or other type, try to find a submodule
                if crate_name in submodule_map:
                    output_lines.append(f'{crate_name} = {{ path = "{submodule_map[crate_name]}" }}')
                else:
                    # If no submodule found, keep the original line (version dependency)
                    output_lines.append(line)

    return "\n".join(output_lines)

if __name__ == "__main__":
    tt_txt_path = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tt.txt"
    project_root = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
    submodules_dir = os.path.join(project_root, "submodules")
    
    processed_content = process_tt_txt_accurate(tt_txt_path, submodules_dir)
    print(processed_content)
