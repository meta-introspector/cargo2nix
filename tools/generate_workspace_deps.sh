#!/bin/bash

ROOT_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix"
SUBMODULE_NAMES_FILE="${ROOT_DIR}/submodule_names.txt"
ALL_DEPS_FILE="${ROOT_DIR}/all_deps.txt"
GENERATED_WORKSPACE_DEPS_FILE="${ROOT_DIR}/generated_workspace_deps.toml"

# Clear the output file
> "${GENERATED_WORKSPACE_DEPS_FILE}"

echo "[workspace.dependencies]" >> "${GENERATED_WORKSPACE_DEPS_FILE}"

# Read submodule names into an array
readarray -t SUBMODULE_NAMES < "${SUBMODULE_NAMES_FILE}"

# Process all_deps.txt
while IFS= read -r line; do
    # Extract package name and version
    # Example: "toml_edit 0.23.7" -> name="toml_edit", version="0.23.7"
    # Example: "anyhow" -> name="anyhow", version=""
    if [[ "$line" =~ ^([^[:space:]]+)[[:space:]]+([0-9\.]+)\] ]]; then
        package_name="${BASH_REMATCH[1]}"
        package_version="${BASH_REMATCH[2]}"
    else
        package_name="$line"
        package_version=""
    fi

    # Check if it's a submodule
    is_submodule=false
    for submodule_name in "${SUBMODULE_NAMES[@]}"; do
        if [[ "$package_name" == "$submodule_name" ]]; then
            is_submodule=true
            break
        fi
    done

    if [[ "$is_submodule" == true ]]; then
        # Handle special cases for submodules that are nested or have different names
        case "$package_name" in
            "time")
                echo "${package_name} = { path = \"./submodules/time-rs/time\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "time-core")
                echo "${package_name} = { path = \"./submodules/time-rs/time-core\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "time-macros")
                echo "${package_name} = { path = \"./submodules/time-rs/time-macros\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "deranged")
                echo "${package_name} = { path = \"./submodules/deranged/deranged\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "powerfmt")
                echo "${package_name} = { path = \"./submodules/powerfmt/powerfmt\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "powerfmt-macros")
                echo "${package_name} = { path = \"./submodules/powerfmt/powerfmt-macros\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde")
                echo "${package_name} = { path = \"./submodules/serde/serde\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde_core")
                echo "${package_name} = { path = \"./submodules/serde/serde_core\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde_derive")
                echo "${package_name} = { path = \"./submodules/serde/serde_derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cargo")
                echo "${package_name} = { path = \"./submodules/cargo\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clap")
                echo "${package_name} = { path = \"./submodules/clap\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clap_builder")
                echo "${package_name} = { path = \"./submodules/clap/clap_builder\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clap_derive")
                echo "${package_name} = { path = \"./submodules/clap/clap_derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clap_lex")
                echo "${package_name} = { path = \"./submodules/clap/clap_lex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-ini")
                echo "${package_name} = { path = \"./submodules/rust-ini\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ordered-multimap")
                echo "${package_name} = { path = \"./submodules/ordered-multimap-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "heapless")
                echo "${package_name} = { path = \"./submodules/heapless\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "faster-hex")
                echo "${package_name} = { path = \"./submodules/faster-hex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dlv-list")
                echo "${package_name} = { path = \"./submodules/dlv-list-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "allocator-api2")
                echo "${package_name} = { path = \"./submodules/allocator-api2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hashbrown")
                echo "${package_name} = { path = \"./submodules/hashbrown\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ron")
                echo "${package_name} = { path = \"./submodules/ron-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "config")
                echo "${package_name} = { path = \"./submodules/config-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dashmap")
                echo "${package_name} = { path = \"./submodules/dashmap\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "base64")
                echo "${package_name} = { path = \"./submodules/rust-base64\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "proc-macro2")
                echo "${package_name} = { path = \"./submodules/proc-macro2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "quote")
                echo "${package_name} = { path = \"./submodules/quote\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "syn")
                echo "${package_name} = { path = \"./submodules/syn\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rand")
                echo "${package_name} = { path = \"./submodules/rand\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "criterion")
                echo "${package_name} = { path = \"./submodules/criterion\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "libc")
                echo "${package_name} = { path = \"./submodules/libc\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "addr2line")
                echo "${package_name} = { path = \"./submodules/addr2line\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "anyhow")
                echo "${package_name} = { path = \"./submodules/anyhow\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "arc-swap")
                echo "${package_name} = { path = \"./submodules/arc-swap\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "arraydeque")
                echo "${package_name} = { path = \"./submodules/arraydeque\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "arrayref")
                echo "${package_name} = { path = \"./submodules/arrayref\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "arrayvec")
                echo "${package_name} = { path = \"./submodules/arrayvec\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-attributes")
                echo "${package_name} = { path = \"./submodules/async-attributes\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-channel")
                echo "${package_name} = { path = \"./submodules/async-channel\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-executor")
                echo "${package_name} = { path = \"./submodules/async-executor\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-global-executor")
                echo "${package_name} = { path = \"./submodules/async-global-executor\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-io")
                echo "${package_name} = { path = \"./submodules/async-io\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-lock")
                echo "${package_name} = { path = \"./submodules/async-lock\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-net")
                echo "${package_name} = { path = \"./submodules/async-net\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-std")
                echo "${package_name} = { path = \"./submodules/async-std\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-task")
                echo "${package_name} = { path = \"./submodules/async-task\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "async-trait")
                echo "${package_name} = { path = \"./submodules/async-trait\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "atomic-waker")
                echo "${package_name} = { path = \"./submodules/atomic-waker\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "atty")
                echo "${package_name} = { path = \"./submodules/atty\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "autocfg")
                echo "${package_name} = { path = \"./submodules/autocfg\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "aws-lc-rs")
                echo "${package_name} = { path = \"./submodules/aws-lc-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "backtrace")
                echo "${package_name} = { path = \"./submodules/backtrace-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bit-set")
                echo "${package_name} = { path = \"./submodules/bit-set\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bit-vec")
                echo "${package_name} = { path = \"./submodules/bit-vec\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bitflags")
                echo "${package_name} = { path = \"./submodules/bitflags\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "blink-alloc")
                echo "${package_name} = { path = \"./submodules/blink-alloc\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "blocking")
                echo "${package_name} = { path = \"./submodules/blocking\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bstr")
                echo "${package_name} = { path = \"./submodules/bstr\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bumpalo")
                echo "${package_name} = { path = \"./submodules/bumpalo\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "byteorder")
                echo "${package_name} = { path = \"./submodules/byteorder\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "bytes")
                echo "${package_name} = { path = \"./submodules/bytes\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cassowary-rs")
                echo "${package_name} = { path = \"./submodules/cassowary-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "castaway")
                echo "${package_name} = { path = \"./submodules/castaway\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cbor")
                echo "${package_name} = { path = \"./submodules/cbor\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cc-rs")
                echo "${package_name} = { path = \"./submodules/cc-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cesu8-rs")
                echo "${package_name} = { path = \"./submodules/cesu8-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cfg-if")
                echo "${package_name} = { path = \"./submodules/cfg-if\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cfg_aliases")
                echo "${package_name} = { path = \"./submodules/cfg_aliases\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "chrono")
                echo "${package_name} = { path = \"./submodules/chrono\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clang-sys")
                echo "${package_name} = { path = \"./submodules/clang-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "clru-rs")
                echo "${package_name} = { path = \"./submodules/clru-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cmake-rs")
                echo "${package_name} = { path = \"./submodules/cmake-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "colorify")
                echo "${package_name} = { path = \"./submodules/colorify\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "combine")
                echo "${package_name} = { path = \"./submodules/combine\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "compact_str")
                echo "${package_name} = { path = \"./submodules/compact_str\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "compiler-builtins")
                echo "${package_name} = { path = \"./submodules/compiler-builtins\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "conpty")
                echo "${package_name} = { path = \"./submodules/conpty\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "console")
                echo "${package_name} = { path = \"./submodules/console\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "constrandom")
                echo "${package_name} = { path = \"./submodules/constrandom\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "content_inspector")
                echo "${package_name} = { path = \"./submodules/content_inspector\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "convert-case")
                echo "${package_name} = { path = \"./submodules/convert-case\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "core-foundation-rs")
                echo "${package_name} = { path = \"./submodules/core-foundation-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "corn")
                echo "${package_name} = { path = \"./submodules/corn\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "coverage-helper")
                echo "${package_name} = { path = \"./submodules/coverage-helper\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "crc-catalog")
                echo "${package_name} = { path = \"./submodules/crc-catalog\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "crc-rs")
                echo "${package_name} = { path = \"./submodules/crc-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "criterion.rs")
                echo "${package_name} = { path = \"./submodules/criterion.rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "critical-section")
                echo "${package_name} = { path = \"./submodules/critical-section\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "crossbeam")
                echo "${package_name} = { path = \"./submodules/crossbeam\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "crossterm")
                echo "${package_name} = { path = \"./submodules/crossterm\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "cryptocorrosion")
                echo "${package_name} = { path = \"./submodules/cryptocorrosion\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "curl-rust")
                echo "${package_name} = { path = \"./submodules/curl-rust\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "darling")
                echo "${package_name} = { path = \"./submodules/darling\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dbus-rs")
                echo "${package_name} = { path = \"./submodules/dbus-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "defer")
                echo "${package_name} = { path = \"./submodules/defer\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "defmt")
                echo "${package_name} = { path = \"./submodules/defmt\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "diff")
                echo "${package_name} = { path = \"./submodules/diff\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "displaydoc")
                echo "${package_name} = { path = \"./submodules/displaydoc\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dissimilar")
                echo "${package_name} = { path = \"./submodules/dissimilar\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dlv-list-rs")
                echo "${package_name} = { path = \"./submodules/dlv-list-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "document-features")
                echo "${package_name} = { path = \"./submodules/document-features\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "duncer")
                echo "${package_name} = { path = \"./submodules/duncer\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "dyn-clone")
                echo "${package_name} = { path = \"./submodules/dyn-clone\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "either")
                echo "${package_name} = { path = \"./submodules/either\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "elliptic-curves")
                echo "${package_name} = { path = \"./submodules/elliptic-curves\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "embedded-hal")
                echo "${package_name} = { path = \"./submodules/embedded-hal\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "encode_unicode")
                echo "${package_name} = { path = \"./submodules/encode_unicode\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "encoding_rs")
                echo "${package_name} = { path = \"./submodules/encoding_rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "env_logger")
                echo "${package_name} = { path = \"./submodules/env_logger\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "equivalent")
                echo "${package_name} = { path = \"./submodules/equivalent\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "erased-serde")
                echo "${package_name} = { path = \"./submodules/erased-serde\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "escargot")
                echo "${package_name} = { path = \"./submodules/escargot\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "event-listener")
                echo "${package_name} = { path = \"./submodules/event-listener\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "event-listener-strategy")
                echo "${package_name} = { path = \"./submodules/event-listener-strategy\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "expectrl")
                echo "${package_name} = { path = \"./submodules/expectrl\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fallible-streaming-iterator")
                echo "${package_name} = { path = \"./submodules/fallible-streaming-iterator\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fastrand")
                echo "${package_name} = { path = \"./submodules/fastrand\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ff")
                echo "${package_name} = { path = \"./submodules/ff\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fiat-crypto")
                echo "${package_name} = { path = \"./submodules/fiat-crypto\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "filetime")
                echo "${package_name} = { path = \"./submodules/filetime\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "flate2-rs")
                echo "${package_name} = { path = \"./submodules/flate2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "float-cmp")
                echo "${package_name} = { path = \"./submodules/float-cmp\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "foldhash")
                echo "${package_name} = { path = \"./submodules/foldhash\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "foreign-types")
                echo "${package_name} = { path = \"./submodules/foreign-types\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "formats")
                echo "${package_name} = { path = \"./submodules/formats\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fs_extra")
                echo "${package_name} = { path = \"./submodules/fs_extra\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fs-err")
                echo "${package_name} = { path = \"./submodules/fs-err\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "fsevent-rust")
                echo "${package_name} = { path = \"./submodules/fsevent-rust\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "futures-lite")
                echo "${package_name} = { path = \"./submodules/futures-lite\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "futures-rs")
                echo "${package_name} = { path = \"./submodules/futures-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "futures-timer")
                echo "${package_name} = { path = \"./submodules/futures-timer\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "generic-array")
                echo "${package_name} = { path = \"./submodules/generic-array\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "gg-alloc")
                echo "${package_name} = { path = \"./submodules/gg-alloc\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "gimli")
                echo "${package_name} = { path = \"./submodules/gimli\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "git2-rs")
                echo "${package_name} = { path = \"./submodules/git2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "gitoxide")
                echo "${package_name} = { path = \"./submodules/gitoxide\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "glob")
                echo "${package_name} = { path = \"./submodules/glob\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "globwalk")
                echo "${package_name} = { path = \"./submodules/globwalk\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "gloo")
                echo "${package_name} = { path = \"./submodules/gloo\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "group")
                echo "${package_name} = { path = \"./submodules/group\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "h2")
                echo "${package_name} = { path = \"./submodules/h2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "half-rs")
                echo "${package_name} = { path = \"./submodules/half-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "handlebars-rust")
                echo "${package_name} = { path = \"./submodules/handlebars-rust\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hash32")
                echo "${package_name} = { path = \"./submodules/hash32\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hashes")
                echo "${package_name} = { path = \"./submodules/hashes\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hashlink")
                echo "${package_name} = { path = \"./submodules/hashlink\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "headers")
                echo "${package_name} = { path = \"./submodules/headers\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "heck")
                echo "${package_name} = { path = \"./submodules/heck\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hermit-rs")
                echo "${package_name} = { path = \"./submodules/hermit-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "html-escape")
                echo "${package_name} = { path = \"./submodules/html-escape\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "http")
                echo "${package_name} = { path = \"./submodules/http\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "http-auth")
                echo "${package_name} = { path = \"./submodules/http-auth\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "http-body")
                echo "${package_name} = { path = \"./submodules/http-body\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "httparse")
                echo "${package_name} = { path = \"./submodules/httparse\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "httpdate")
                echo "${package_name} = { path = \"./submodules/httpdate\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "human-format-rs")
                echo "${package_name} = { path = \"./submodules/human-format-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hyper")
                echo "${package_name} = { path = \"./submodules/hyper\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hyper-rustls")
                echo "${package_name} = { path = \"./submodules/hyper-rustls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hyper-tls")
                echo "${package_name} = { path = \"./submodules/hyper-tls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "hyper-util")
                echo "${package_name} = { path = \"./submodules/hyper-util\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "iana-time-zone")
                echo "${package_name} = { path = \"./submodules/iana-time-zone\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "icu4x")
                echo "${package_name} = { path = \"./submodules/icu4x\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ident_case")
                echo "${package_name} = { path = \"./submodules/ident_case\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "idna_adapter")
                echo "${package_name} = { path = \"./submodules/idna_adapter\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "im-rs")
                echo "${package_name} = { path = \"./submodules/im-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "imara-diff")
                echo "${package_name} = { path = \"./submodules/imara-diff\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "inotify")
                echo "${package_name} = { path = \"./submodules/inotify\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "inotify-sys")
                echo "${package_name} = { path = \"./submodules/inotify-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "insta")
                echo "${package_name} = { path = \"./submodules/insta\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "instant")
                echo "${package_name} = { path = \"./submodules/instant\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "inventory")
                echo "${package_name} = { path = \"./submodules/inventory\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "io-uring")
                echo "${package_name} = { path = \"./submodules/io-uring\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ipnet")
                echo "${package_name} = { path = \"./submodules/ipnet\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "iri-string")
                echo "${package_name} = { path = \"./submodules/iri-string\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is_ci")
                echo "${package_name} = { path = \"./submodules/is_ci\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is_executable")
                echo "${package_name} = { path = \"./submodules/is_executable\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is_terminal_polyfill")
                echo "${package_name} = { path = \"./submodules/is_terminal_polyfill\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is-docker")
                echo "${package_name} = { path = \"./submodules/is-docker\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is-terminal")
                echo "${package_name} = { path = \"./submodules/is-terminal\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "is-wsl")
                echo "${package_name} = { path = \"./submodules/is-wsl\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "itertools")
                echo "${package_name} = { path = \"./submodules/itertools\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "itertools-num")
                echo "${package_name} = { path = \"./submodules/itertools-num\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "itoa")
                echo "${package_name} = { path = \"./submodules/itoa\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "jiff")
                echo "${package_name} = { path = \"./submodules/jiff\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "jni-rs")
                echo "${package_name} = { path = \"./submodules/jni-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "jni-sys")
                echo "${package_name} = { path = \"./submodules/jni-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "jobserver-rs")
                echo "${package_name} = { path = \"./submodules/jobserver-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "json")
                echo "${package_name} = { path = \"./submodules/json\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "json5-rs")
                echo "${package_name} = { path = \"./submodules/json5-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "jwalk")
                echo "${package_name} = { path = \"./submodules/jwalk\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "KDFs")
                echo "${package_name} = { path = \"./submodules/KDFs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "kernel")
                echo "${package_name} = { path = \"./submodules/kernel\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "kstring")
                echo "${package_name} = { path = \"./submodules/kstring\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "kv-log-macro")
                echo "${package_name} = { path = \"./submodules/kv-log-macro\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "layout")
                echo "${package_name} = { path = \"./submodules/layout\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "lazy-static")
                echo "${package_name} = { path = \"./submodules/lazy-static\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "lazycell")
                echo "${package_name} = { path = \"./submodules/lazycell\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "leb128")
                echo "${package_name} = { path = \"./submodules/leb128\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "libhermit-rs")
                echo "${package_name} = { path = \"./submodules/libhermit-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "libz-sys")
                echo "${package_name} = { path = \"./submodules/libz-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "linux-raw-sys")
                echo "${package_name} = { path = \"./submodules/linux-raw-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "litrs")
                echo "${package_name} = { path = \"./submodules/litrs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "log")
                echo "${package_name} = { path = \"./submodules/log\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "lru-rs")
                echo "${package_name} = { path = \"./submodules/lru-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "lru-slab")
                echo "${package_name} = { path = \"./submodules/lru-slab\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "MACs")
                echo "${package_name} = { path = \"./submodules/MACs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "maplit")
                echo "${package_name} = { path = \"./submodules/maplit\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "matchers")
                echo "${package_name} = { path = \"./submodules/matchers\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "maybe-async-rs")
                echo "${package_name} = { path = \"./submodules/maybe-async-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "md5")
                echo "${package_name} = { path = \"./submodules/md5\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "memchr")
                echo "${package_name} = { path = \"./submodules/memchr\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "memmap2-rs")
                echo "${package_name} = { path = \"./submodules/memmap2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "memoffset")
                echo "${package_name} = { path = \"./submodules/memoffset\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "mime")
                echo "${package_name} = { path = \"./submodules/mime\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "mime_guess")
                echo "${package_name} = { path = \"./submodules/mime_guess\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "minicov")
                echo "${package_name} = { path = \"./submodules/minicov\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "minimal-lexical")
                echo "${package_name} = { path = \"./submodules/minimal-lexical\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "miniz_oxide")
                echo "${package_name} = { path = \"./submodules/miniz_oxide\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "mio")
                echo "${package_name} = { path = \"./submodules/mio\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "miow")
                echo "${package_name} = { path = \"./submodules/miow\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "miri")
                echo "${package_name} = { path = \"./submodules/miri\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "nghttp2-rs")
                echo "${package_name} = { path = \"./submodules/nghttp2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "nix")
                echo "${package_name} = { path = \"./submodules/nix\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "no-panic")
                echo "${package_name} = { path = \"./submodules/no-panic\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "nom")
                echo "${package_name} = { path = \"./submodules/nom\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "normalize-line-endings")
                echo "${package_name} = { path = \"./submodules/normalize-line-endings\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "normpath")
                echo "${package_name} = { path = \"./submodules/normpath\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "notify")
                echo "${package_name} = { path = \"./submodules/notify\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ntapi")
                echo "${package_name} = { path = \"./submodules/ntapi\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "nu-ansi-term")
                echo "${package_name} = { path = \"./submodules/nu-ansi-term\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "num-complex")
                echo "${package_name} = { path = \"./submodules/num-complex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "num-conv")
                echo "${package_name} = { path = \"./submodules/num-conv\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "num-modular")
                echo "${package_name} = { path = \"./submodules/num-modular\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "num-order")
                echo "${package_name} = { path = \"./submodules/num-order\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "num-traits")
                echo "${package_name} = { path = \"./submodules/num-traits\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "objc2")
                echo "${package_name} = { path = \"./submodules/objc2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "object")
                echo "${package_name} = { path = \"./submodules/object\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "once_cell")
                echo "${package_name} = { path = \"./submodules/once_cell\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "once_cell_polyfill")
                echo "${package_name} = { path = \"./submodules/once_cell_polyfill\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "oorandom")
                echo "${package_name} = { path = \"./submodules/oorandom\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "open-rs")
                echo "${package_name} = { path = \"./submodules/open-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "opener")
                echo "${package_name} = { path = \"./submodules/opener\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "openssl-probe")
                echo "${package_name} = { path = \"./submodules/openssl-probe\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "openssl-src-rs")
                echo "${package_name} = { path = \"./submodules/openssl-src-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "option_set")
                echo "${package_name} = { path = \"./submodules/option_set\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ordered-multimap-rs")
                echo "${package_name} = { path = \"./submodules/ordered-multimap-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "orion")
                echo "${package_name} = { path = \"./submodules/orion\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "os_info")
                echo "${package_name} = { path = \"./submodules/os_info\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "os_str_bytes")
                echo "${package_name} = { path = \"./submodules/os_str_bytes\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "parking")
                echo "${package_name} = { path = \"./submodules/parking\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "parking_lot")
                echo "${package_name} = { path = \"./submodules/parking_lot\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "partial_ref")
                echo "${package_name} = { path = \"./submodules/partial_ref\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pasetors")
                echo "${package_name} = { path = \"./submodules/pasetors\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "paste")
                echo "${package_name} = { path = \"./submodules/paste\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "permutohedron")
                echo "${package_name} = { path = \"./submodules/permutohedron\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pest")
                echo "${package_name} = { path = \"./submodules/pest\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pin-project")
                echo "${package_name} = { path = \"./submodules/pin-project\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pin-project-lite")
                echo "${package_name} = { path = \"./submodules/pin-project-lite\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pin-utils")
                echo "${package_name} = { path = \"./submodules/pin-utils\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "piper")
                echo "${package_name} = { path = \"./submodules/piper\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pkg-config-rs")
                echo "${package_name} = { path = \"./submodules/pkg-config-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pki-types")
                echo "${package_name} = { path = \"./submodules/pki-types\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "plotters")
                echo "${package_name} = { path = \"./submodules/plotters\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "polling")
                echo "${package_name} = { path = \"./submodules/polling\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "portable-atomic")
                echo "${package_name} = { path = \"./submodules/portable-atomic\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "portable-simd")
                echo "${package_name} = { path = \"./submodules/portable-simd\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "prettyplease")
                echo "${package_name} = { path = \"./submodules/prettyplease\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "proc-macro-error-2")
                echo "${package_name} = { path = \"./submodules/proc-macro-error-2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "prodash")
                echo "${package_name} = { path = \"./submodules/prodash\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "proptest")
                echo "${package_name} = { path = \"./submodules/proptest\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ptyprocess")
                echo "${package_name} = { path = \"./submodules/ptyprocess\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "pulldown-cmark")
                echo "${package_name} = { path = \"./submodules/pulldown-cmark\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "quick-error")
                echo "${package_name} = { path = \"./submodules/quick-error\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "quick-xml")
                echo "${package_name} = { path = \"./submodules/quick-xml\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "quinn")
                echo "${package_name} = { path = \"./submodules/quinn\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "r-efi")
                echo "${package_name} = { path = \"./submodules/r-efi\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ratatui")
                echo "${package_name} = { path = \"./submodules/ratatui\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rayon")
                echo "${package_name} = { path = \"./submodules/rayon\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ref-cast")
                echo "${package_name} = { path = \"./submodules/ref-cast\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "regex")
                echo "${package_name} = { path = \"./submodules/regex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "relative-path")
                echo "${package_name} = { path = \"./submodules/relative-path\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "reqwest")
                echo "${package_name} = { path = \"./submodules/reqwest\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ring")
                echo "${package_name} = { path = \"./submodules/ring\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ripgrep")
                echo "${package_name} = { path = \"./submodules/ripgrep\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rngs")
                echo "${package_name} = { path = \"./submodules/rngs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rstest")
                echo "${package_name} = { path = \"./submodules/rstest\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rusqlite")
                echo "${package_name} = { path = \"./submodules/rusqlite\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust")
                echo "${package_name} = { path = \"./submodules/rust\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-analyzer")
                echo "${package_name} = { path = \"./submodules/rust-analyzer\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-base64")
                echo "${package_name} = { path = \"./submodules/rust-base64\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-bindgen")
                echo "${package_name} = { path = \"./submodules/rust-bindgen\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-cexpr")
                echo "${package_name} = { path = \"./submodules/rust-cexpr\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-clippy")
                echo "${package_name} = { path = \"./submodules/rust-clippy\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-crc32fast")
                echo "${package_name} = { path = \"./submodules/rust-crc32fast\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-csv")
                echo "${package_name} = { path = \"./submodules/rust-csv\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-ct-codecs")
                echo "${package_name} = { path = \"./submodules/rust-ct-codecs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-derive-builder")
                echo "${package_name} = { path = \"./submodules/rust-derive-builder\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-ed25519-compact")
                echo "${package_name} = { path = \"./submodules/rust-ed25519-compact\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-errno")
                echo "${package_name} = { path = \"./submodules/rust-errno\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-fallible-iterator")
                echo "${package_name} = { path = \"./submodules/rust-fallible-iterator\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-fnv")
                echo "${package_name} = { path = \"./submodules/rust-fnv\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-hex")
                echo "${package_name} = { path = \"./submodules/rust-hex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-ini")
                echo "${package_name} = { path = \"./submodules/rust-ini\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-jni-sys")
                echo "${package_name} = { path = \"./submodules/rust-jni-sys\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-native-tls")
                echo "${package_name} = { path = \"./submodules/rust-native-tls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-openssl")
                echo "${package_name} = { path = \"./submodules/rust-openssl\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-ordered-float")
                echo "${package_name} = { path = \"./submodules/rust-ordered-float\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-plist")
                echo "${package_name} = { path = \"./submodules/rust-plist\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-pretty-assertions")
                echo "${package_name} = { path = \"./submodules/rust-pretty-assertions\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-security-framework")
                echo "${package_name} = { path = \"./submodules/rust-security-framework\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-shlex")
                echo "${package_name} = { path = \"./submodules/rust-shlex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-smallvec")
                echo "${package_name} = { path = \"./submodules/rust-smallvec\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rust-url")
                echo "${package_name} = { path = \"./submodules/rust-url\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustc-demangle")
                echo "${package_name} = { path = \"./submodules/rustc-demangle\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustc-hash")
                echo "${package_name} = { path = \"./submodules/rustc-hash\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustc-hex")
                echo "${package_name} = { path = \"./submodules/rustc-hex\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustc-stable-hash")
                echo "${package_name} = { path = \"./submodules/rustc-stable-hash\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustc-version-rs")
                echo "${package_name} = { path = \"./submodules/rustc-version-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustfmt")
                echo "${package_name} = { path = \"./submodules/rustfmt\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustix")
                echo "${package_name} = { path = \"./submodules/rustix\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustls")
                echo "${package_name} = { path = \"./submodules/rustls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustls-ffi")
                echo "${package_name} = { path = \"./submodules/rustls-ffi\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustls-native-certs")
                echo "${package_name} = { path = \"./submodules/rustls-native-certs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustls-platform-verifier")
                echo "${package_name} = { path = \"./submodules/rustls-platform-verifier\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rustversion")
                echo "${package_name} = { path = \"./submodules/rustversion\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "rusty-fork")
                echo "${package_name} = { path = \"./submodules/rusty-fork\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ryu")
                echo "${package_name} = { path = \"./submodules/ryu\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "same-file")
                echo "${package_name} = { path = \"./submodules/same-file\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "scalable-concurrent-containers")
                echo "${package_name} = { path = \"./submodules/scalable-concurrent-containers\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "scalable-delayed-dealloc")
                echo "${package_name} = { path = \"./submodules/scalable-delayed-dealloc\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "schannel-rs")
                echo "${package_name} = { path = \"./submodules/schannel-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "schemars")
                echo "${package_name} = { path = \"./submodules/schemars\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "scoped-tls")
                echo "${package_name} = { path = \"./submodules/scoped-tls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "scopeguard")
                echo "${package_name} = { path = \"./submodules/scopeguard\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde_urlencoded")
                echo "${package_name} = { path = \"./submodules/serde_urlencoded\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde-ignored")
                echo "${package_name} = { path = \"./submodules/serde-ignored\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde-stacker")
                echo "${package_name} = { path = \"./submodules/serde-stacker\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde-untagged")
                echo "${package_name} = { path = \"./submodules/serde-untagged\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serde-value")
                echo "${package_name} = { path = \"./submodules/serde-value\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "serial_test")
                echo "${package_name} = { path = \"./submodules/serial_test\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "sha1-smol")
                echo "${package_name} = { path = \"./submodules/sha1-smol\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "sharded-slab")
                echo "${package_name} = { path = \"./submodules/sharded-slab\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "shell-escape")
                echo "${package_name} = { path = \"./submodules/shell-escape\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "shell-words")
                echo "${package_name} = { path = \"./submodules/shell-words\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "signal-hook")
                echo "${package_name} = { path = \"./submodules/signal-hook\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "signatures")
                echo "${package_name} = { path = \"./submodules/signatures\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "simd-adler32")
                echo "${package_name} = { path = \"./submodules/simd-adler32\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "similar")
                echo "${package_name} = { path = \"./submodules/similar\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "sized-chunks")
                echo "${package_name} = { path = \"./submodules/sized-chunks\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "slab")
                echo "${package_name} = { path = \"./submodules/slab\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "smol")
                echo "${package_name} = { path = \"./submodules/smol\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "snapbox")
                echo "${package_name} = { path = \"./submodules/snapbox\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "socket2")
                echo "${package_name} = { path = \"./submodules/socket2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ssh2-rs")
                echo "${package_name} = { path = \"./submodules/ssh2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "stability")
                echo "${package_name} = { path = \"./submodules/stability\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "stable_deref_trait")
                echo "${package_name} = { path = \"./submodules/stable_deref_trait\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "stacker")
                echo "${package_name} = { path = \"./submodules/stacker\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "static-assertions-rs")
                echo "${package_name} = { path = \"./submodules/static-assertions-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "stdarch")
                echo "${package_name} = { path = \"./submodules/stdarch\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "strsim-rs")
                echo "${package_name} = { path = \"./submodules/strsim-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "strum")
                echo "${package_name} = { path = \"./submodules/strum\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "subtle")
                echo "${package_name} = { path = \"./submodules/subtle\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "supports-hyperlinks")
                echo "${package_name} = { path = \"./submodules/supports-hyperlinks\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "supports-unicode")
                echo "${package_name} = { path = \"./submodules/supports-unicode\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "sync_wrapper")
                echo "${package_name} = { path = \"./submodules/sync_wrapper\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "sysinfo")
                echo "${package_name} = { path = \"./submodules/sysinfo\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "system-configuration-rs")
                echo "${package_name} = { path = \"./submodules/system-configuration-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tar-rs")
                echo "${package_name} = { path = \"./submodules/tar-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "target-triple")
                echo "${package_name} = { path = \"./submodules/target-triple\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "temp-env")
                echo "${package_name} = { path = \"./submodules/temp-env\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tempfile")
                echo "${package_name} = { path = \"./submodules/tempfile\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tera")
                echo "${package_name} = { path = \"./submodules/tera\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "termcolor")
                echo "${package_name} = { path = \"./submodules/termcolor\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "terminal-size")
                echo "${package_name} = { path = \"./submodules/terminal-size\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "termtree")
                echo "${package_name} = { path = \"./submodules/termtree\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "test")
                echo "${package_name} = { path = \"./submodules/test\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "text-size")
                echo "${package_name} = { path = \"./submodules/text-size\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "textwrap")
                echo "${package_name} = { path = \"./submodules/textwrap\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "thread_local-rs")
                echo "${package_name} = { path = \"./submodules/thread_local-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tiny-keccak")
                echo "${package_name} = { path = \"./submodules/tiny-keccak\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "TinyTemplate")
                echo "${package_name} = { path = \"./submodules/TinyTemplate\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tinyvec")
                echo "${package_name} = { path = \"./submodules/tinyvec\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tinyvec_macros")
                echo "${package_name} = { path = \"./submodules/tinyvec_macros\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tls")
                echo "${package_name} = { path = \"./submodules/tls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tokio")
                echo "${package_name} = { path = \"./submodules/tokio\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tokio-rustls")
                echo "${package_name} = { path = \"./submodules/tokio-rustls\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "toml")
                echo "${package_name} = { path = \"./submodules/toml\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tower")
                echo "${package_name} = { path = \"./submodules/tower\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tower-http")
                echo "${package_name} = { path = \"./submodules/tower-http\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tracing")
                echo "${package_name} = { path = \"./submodules/tracing\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tracing-chrome")
                echo "${package_name} = { path = \"./submodules/tracing-chrome\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tracing-forest")
                echo "${package_name} = { path = \"./submodules/tracing-forest\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "traits")
                echo "${package_name} = { path = \"./submodules/traits\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "try-lock")
                echo "${package_name} = { path = \"./submodules/try-lock\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "trybuild")
                echo "${package_name} = { path = \"./submodules/trybuild\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "trycmd")
                echo "${package_name} = { path = \"./submodules/trycmd\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "tui-crates")
                echo "${package_name} = { path = \"./submodules/tui-crates\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "typeid")
                echo "${package_name} = { path = \"./submodules/typeid\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "typenum")
                echo "${package_name} = { path = \"./submodules/typenum\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "typesize")
                echo "${package_name} = { path = \"./submodules/typesize\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "typetag")
                echo "${package_name} = { path = \"./submodules/typetag\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ucd-generate")
                echo "${package_name} = { path = \"./submodules/ucd-generate\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ufmt")
                echo "${package_name} = { path = \"./submodules/ufmt\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "uluru")
                echo "${package_name} = { path = \"./submodules/uluru\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unarray")
                echo "${package_name} = { path = \"./submodules/unarray\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "ungrammar")
                echo "${package_name} = { path = \"./submodules/ungrammar\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicase")
                echo "${package_name} = { path = \"./submodules/unicase\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-bom")
                echo "${package_name} = { path = \"./submodules/unicode-bom\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-ident")
                echo "${package_name} = { path = \"./submodules/unicode-ident\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-normalization")
                echo "${package_name} = { path = \"./submodules/unicode-normalization\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-segmentation")
                echo "${package_name} = { path = \"./submodules/unicode-segmentation\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-truncate")
                echo "${package_name} = { path = \"./submodules/unicode-truncate\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-width")
                echo "${package_name} = { path = \"./submodules/unicode-width\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "unicode-xid")
                echo "${package_name} = { path = \"./submodules/unicode-xid\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "untrusted")
                echo "${package_name} = { path = \"./submodules/untrusted\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "utf16_iter")
                echo "${package_name} = { path = \"./submodules/utf16_iter\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "utf8_iter")
                echo "${package_name} = { path = \"./submodules/utf8_iter\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "utf8-width")
                echo "${package_name} = { path = \"./submodules/utf8-width\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "utils")
                echo "${package_name} = { path = \"./submodules/utils\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "valuable")
                echo "${package_name} = { path = \"./submodules/valuable\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "value-bag")
                echo "${package_name} = { path = \"./submodules/value-bag\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "varisat")
                echo "${package_name} = { path = \"./submodules/varisat\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "vcpkg-rs")
                echo "${package_name} = { path = \"./submodules/vcpkg-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "vec_mut_scan")
                echo "${package_name} = { path = \"./submodules/vec_mut_scan\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "version_check")
                echo "${package_name} = { path = \"./submodules/version_check\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "vte")
                echo "${package_name} = { path = \"./submodules/vte\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "wait-timeout")
                echo "${package_name} = { path = \"./submodules/wait-timeout\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "walkdir")
                echo "${package_name} = { path = \"./submodules/walkdir\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "want")
                echo "${package_name} = { path = \"./submodules/want\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "warp")
                echo "${package_name} = { path = \"./submodules/warp\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "wasi")
                echo "${package_name} = { path = \"./submodules/wasi\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "wasi-rs")
                echo "${package_name} = { path = \"./submodules/wasi-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "wasm-bindgen")
                echo "${package_name} = { path = \"./submodules/wasm-bindgen\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "web-time")
                echo "${package_name} = { path = \"./submodules/web-time\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "webpki")
                echo "${package_name} = { path = \"./submodules/webpki\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "webpki-roots")
                echo "${package_name} = { path = \"./submodules/webpki-roots\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "winapi-rs")
                echo "${package_name} = { path = \"./submodules/winapi-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "winapi-util")
                echo "${package_name} = { path = \"./submodules/winapi-util\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "windows-rs")
                echo "${package_name} = { path = \"./submodules/windows-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "winnow")
                echo "${package_name} = { path = \"./submodules/winnow\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "winreg-rs")
                echo "${package_name} = { path = \"./submodules/winreg-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "wit-bindgen")
                echo "${package_name} = { path = \"./submodules/wit-bindgen\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "write16")
                echo "${package_name} = { path = \"./submodules/write16\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "xattr")
                echo "${package_name} = { path = \"./submodules/xattr\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "xz2-rs")
                echo "${package_name} = { path = \"./submodules/xz2-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "yaml-rust2")
                echo "${package_name} = { path = \"./submodules/yaml-rust2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "yansi")
                echo "${package_name} = { path = \"./submodules/yansi\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "yoke")
                echo "${package_name} = { path = \"./submodules/yoke\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "yoke-derive")
                echo "${package_name} = { path = \"./submodules/yoke-derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerocopy")
                echo "${package_name} = { path = \"./submodules/zerocopy\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerocopy-derive")
                echo "${package_name} = { path = \"./submodules/zerocopy-derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerofrom")
                echo "${package_name} = { path = \"./submodules/zerofrom\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerofrom-derive")
                echo "${package_name} = { path = \"./submodules/zerofrom-derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zeroize")
                echo "${package_name} = { path = \"./submodules/zeroize\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerotrie")
                echo "${package_name} = { path = \"./submodules/zerotrie\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerovec")
                echo "${package_name} = { path = \"./submodules/zerovec\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zerovec-derive")
                echo "${package_name} = { path = \"./submodules/zerovec-derive\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zip2")
                echo "${package_name} = { path = \"./submodules/zip2\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            "zlib-rs")
                echo "${package_name} = { path = \"./submodules/zlib-rs\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
            *)
                echo "${package_name} = { path = \"./submodules/${package_name}\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
                ;; 
        esac
    else
        # Crates.io dependency
        if [[ -n "$package_version" ]]; then
            echo "${package_name} = { version = \"${package_version}\" }" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
        else
            # If no version is specified, use a wildcard or a default version
            echo "${package_name} = \"*\"" >> "${GENERATED_WORKSPACE_DEPS_FILE}"
        fi
    fi
done < "${ALL_DEPS_FILE}"

echo "Generated workspace dependencies written to ${GENERATED_WORKSPACE_DEPS_FILE}"