-*- mode: cargo-compilation; default-directory: "~/cargo2nix/tools/cargo-repo-sync-lib/" -*-
Cargo started at Fri Nov 21 01:42:21

/nix/store/mycvvd0d9ih11ybr7q6iqbfy0wppgj24-rust-default-1.92.0-nightly-2025-09-16/bin/cargo build
warning: resolver for the non root package will be ignored, specify resolver at the workspace root:
package:   /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/crates/cargo2nix/Cargo.toml
workspace: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gimli/Cargo.toml
workspace: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml
warning: resolver for the non root package will be ignored, specify resolver at the workspace root:
package:   /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/bstr/Cargo.toml
workspace: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml
warning: resolver for the non root package will be ignored, specify resolver at the workspace root:
package:   /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/http-auth/Cargo.toml
workspace: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2024 which implies `resolver = "3"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2024 resolver, specify `workspace.resolver = "3"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/colorify/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/Cargo.toml: unused manifest key: workspace.package.resolver
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/zstd-rs/ruzstd/Cargo.toml: `default-features` is ignored for twox-hash, since `default-features` was not specified for `workspace.dependencies.twox-hash`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/zstd-rs/ruzstd/Cargo.toml: unused manifest key: dependencies.fastrand.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/zstd-rs/ruzstd/Cargo.toml: unused manifest key: dependencies.twox-hash.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/zstd-rs/ruzstd/Cargo.toml: unused manifest key: dev-dependencies.rand.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/assert-impl/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/deranged/deranged/Cargo.toml: `default-features` is ignored for powerfmt, since `default-features` was not specified for `workspace.dependencies.powerfmt`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/static-assertions-rs/proc/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/bstr/Cargo.toml: unused manifest key: package.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/regex/Cargo.toml: unused manifest key: package.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/io-uring/Cargo.toml: unused manifest key: package.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/rust-typed-arena/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/quickcheck/Cargo.toml: unused manifest key: package.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/lazy-static/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/memoffset/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/nix/Cargo.toml: unused manifest key: dependencies.memoffset.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/nix/Cargo.toml: unused manifest key: dependencies.pin-utils.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gimli/Cargo.toml: unused manifest key: profile.bench.default-members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gimli/Cargo.toml: unused manifest key: profile.bench.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gimli/Cargo.toml: unused manifest key: profile.bench.resolver
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/equivalent/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/im-rs/rc/Cargo.toml: unused manifest key: package.metadata.docs.rs
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/git2-rs/Cargo.toml: unused manifest key: examples
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: dependencies.cpp_demangle.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: dependencies.serde.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: dev-dependencies.dylib-dep.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: target.cfg(not(all(windows, target_env = "msvc", not(target_vendor = "uwp")))).dependencies.addr2line.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: target.cfg(not(all(windows, target_env = "msvc", not(target_vendor = "uwp")))).dependencies.libc.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: target.cfg(not(all(windows, target_env = "msvc", not(target_vendor = "uwp")))).dependencies.miniz_oxide.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/backtrace-rs/Cargo.toml: unused manifest key: target.cfg(not(all(windows, target_env = "msvc", not(target_vendor = "uwp")))).dependencies.ruzstd.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/glob/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/camino/Cargo.toml: unused manifest key: members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/signal-hook/signal-hook-registry/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/unicode-ident/Cargo.toml: unused manifest key: bench.0.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/test-strategy/Cargo.toml: unused manifest key: dev-dependencies.tokio.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/static-assertions-rs/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/static-assertions-rs/Cargo.toml: unused manifest key: dependencies.proc_static_assertions.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/num_threads/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/shell-escape/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_complete/Cargo.toml: unused manifest key: dependencies.is_executable.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/indexmap/Cargo.toml: unused manifest key: dependencies.serde.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/indexmap/Cargo.toml: unused manifest key: dependencies.serde_core.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/scoped-tls/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.anstream.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.anstyle.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.backtrace.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.strsim.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.terminal_size.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.unicase.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dependencies.unicode-width.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/clap/clap_builder/Cargo.toml: unused manifest key: dev-dependencies.snapbox.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: `default-features` is ignored for reqwest, since `default-features` was not specified for `workspace.dependencies.reqwest`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: `default-features` is ignored for serde, since `default-features` was not specified for `workspace.dependencies.serde`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.async-std.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.async-trait.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.base64.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.curl.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.document-features.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.futures-io.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.futures-lite.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.gix-quote.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.gix-quote.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.pin-project-lite.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.reqwest.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dependencies.serde.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dev-dependencies.async-std.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dev-dependencies.blocking.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/gitoxide/gix-transport/Cargo.toml: unused manifest key: dev-dependencies.maybe-async.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: `default-features` is ignored for mio, since `default-features` was not specified for `workspace.dependencies.mio`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: `default-features` is ignored for io-uring, since `default-features` was not specified for `workspace.dependencies.io-uring`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: `default-features` is ignored for mio, since `default-features` was not specified for `workspace.dependencies.mio`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: `default-features` is ignored for tracing, since `default-features` was not specified for `workspace.dependencies.tracing`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: `default-features` is ignored for nix, since `default-features` was not specified for `workspace.dependencies.nix`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dependencies.bytes.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dependencies.mio.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.futures.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-stream.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-stream.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-test.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-test.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-util.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: dev-dependencies.tokio-util.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(all(tokio_unstable, target_os = "linux")).dependencies.io-uring.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(all(tokio_unstable, target_os = "linux")).dependencies.libc.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(all(tokio_unstable, target_os = "linux")).dependencies.mio.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(all(tokio_unstable, target_os = "linux")).dependencies.slab.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(not(target_family = "wasm")).dependencies.socket2.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(tokio_unstable).dependencies.tracing.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(unix).dependencies.libc.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(unix).dependencies.signal-hook-registry.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(unix).dev-dependencies.libc.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(unix).dev-dependencies.nix.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(windows).dependencies.windows-sys.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/tokio/tokio/Cargo.toml: unused manifest key: target.cfg(windows).dev-dependencies.windows-sys.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/crossbeam/crossbeam-utils/Cargo.toml: unused manifest key: dependencies.atomic-maybe-uninit.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/rustc-demangle/Cargo.toml: no edition set: defaulting to the 2015 edition while the latest is 2024
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/rustc-demangle/Cargo.toml: unused manifest key: package.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/rusqlite/Cargo.toml: unused manifest key: lib.members
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: `default-features` is ignored for cpp_demangle, since `default-features` was not specified for `workspace.dependencies.cpp_demangle`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: `default-features` is ignored for fallible-iterator, since `default-features` was not specified for `workspace.dependencies.fallible-iterator`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: `default-features` is ignored for gimli, since `default-features` was not specified for `workspace.dependencies.gimli`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: `default-features` is ignored for smallvec, since `default-features` was not specified for `workspace.dependencies.smallvec`, this could become a hard error in the future
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.cpp_demangle.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.fallible-iterator.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.gimli.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.memmap2.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.rustc-demangle.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.smallvec.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dependencies.typed-arena.version
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/addr2line/Cargo.toml: unused manifest key: dev-dependencies.auxiliary.path
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/pin-project-lite/Cargo.toml: unused manifest key: dev-dependencies.macrotest.branch
warning: /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/pin-project-lite/Cargo.toml: unused manifest key: dev-dependencies.trybuild.branch
warning: patch for `curl` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
warning: patch for `pasetors` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
warning: patch for `serde` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
warning: patch for `serde_core` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
warning: Patch `mio-aio v1.0.0 (/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/mio-aio)` was not used in the crate graph.
Check that the patched package version and available features are compatible
with the dependency requirements. If the patch has a different version from
what is locked in the Cargo.lock file, run `cargo update` to use the new
version. This may also occur with an optional dependency that is not enabled.
warning: struct `SensibleMoveMask` is never constructed
   --> submodules/memchr/src/vector.rs:118:19
    |
118 | pub(crate) struct SensibleMoveMask(u32);
    |                   ^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `get_for_offset` is never used
   --> submodules/memchr/src/vector.rs:126:8
    |
120 | impl SensibleMoveMask {
    | --------------------- method in this implementation
...
126 |     fn get_for_offset(self) -> u32 {
    |        ^^^^^^^^^^^^^^

warning: `memchr` (lib) generated 2 warnings
warning: unused imports: `ThreadParkerT`, `ThreadParker`, and `UnparkHandleT`
 --> submodules/parking_lot/core/src/parking_lot/mod.rs:7:28
  |
7 | use crate::thread_parker::{ThreadParker, ThreadParkerT, UnparkHandleT};
  |                            ^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `crate::util::UncheckedOptionExt`
 --> submodules/parking_lot/core/src/parking_lot/mod.rs:8:5
  |
8 | use crate::util::UncheckedOptionExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::word_lock::WordLock`
 --> submodules/parking_lot/core/src/parking_lot/mod.rs:9:5
  |
9 | use crate::word_lock::WordLock;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `AtomicPtr`, `AtomicUsize`, `Cell`, `Ordering`, `UnsafeCell`, and `ptr`
  --> submodules/parking_lot/core/src/parking_lot/mod.rs:11:12
   |
11 |     cell::{Cell, UnsafeCell},
   |            ^^^^  ^^^^^^^^^^
12 |     ptr,
   |     ^^^
13 |     sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
   |                    ^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^

warning: unused import: `smallvec::SmallVec`
  --> submodules/parking_lot/core/src/parking_lot/mod.rs:15:5
   |
15 | use smallvec::SmallVec;
   |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `Duration` and `Instant`
  --> submodules/parking_lot/core/src/parking_lot/mod.rs:16:17
   |
16 | use std::time::{Duration, Instant};
   |                 ^^^^^^^^  ^^^^^^^

warning: unused import: `crate::parking_lot::deadlock`
 --> submodules/parking_lot/core/src/parking_lot/thread_data.rs:9:5
  |
9 | use crate::parking_lot::deadlock; // Import the deadlock module
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::time::Duration`
 --> submodules/parking_lot/core/src/parking_lot/hash_table.rs:3:5
  |
3 | use std::time::Duration;
  |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `deadlock`
 --> submodules/parking_lot/core/src/parking_lot/park.rs:1:60
  |
1 | use crate::parking_lot::{lock_bucket, lock_bucket_checked, deadlock, ParkResult, ParkToken, ThreadData};
  |                                                            ^^^^^^^^

warning: unused import: `crate::parking_lot::ThreadData`
 --> submodules/parking_lot/core/src/parking_lot/deadlock.rs:1:5
  |
1 | use crate::parking_lot::ThreadData;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::parking_lot::lock_bucket`
  --> submodules/parking_lot/core/src/word_lock.rs:15:5
   |
15 | use crate::parking_lot::lock_bucket;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `word_lock::ThreadData` is more private than the item `word_lock::with_thread_data`
  --> submodules/parking_lot/core/src/word_lock.rs:51:1
   |
51 | pub fn with_thread_data<T>(f: impl FnOnce(&ThreadData) -> T) -> T {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function `word_lock::with_thread_data` is reachable at visibility `pub(crate)`
   |
note: but type `word_lock::ThreadData` is only usable at visibility `pub(self)`
  --> submodules/parking_lot/core/src/word_lock.rs:16:1
   |
16 | struct ThreadData {
   | ^^^^^^^^^^^^^^^^^
   = note: `#[warn(private_bounds)]` on by default

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:104:1
    |
104 | pub mod park;
    | ^^^^^^^^^^^^
    |
note: the lint level is defined here
   --> submodules/parking_lot/core/src/lib.rs:40:9
    |
 40 | #![warn(missing_docs)]
    |         ^^^^^^^^^^^^

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:107:1
    |
107 | pub mod unpark_one;
    | ^^^^^^^^^^^^^^^^^^

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:110:1
    |
110 | pub mod unpark_all;
    | ^^^^^^^^^^^^^^^^^^

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:113:1
    |
113 | pub mod unpark_requeue;
    | ^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:116:1
    |
116 | pub mod unpark_filter;
    | ^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a module
   --> submodules/parking_lot/core/src/parking_lot/mod.rs:121:1
    |
121 | pub mod deadlock;
    | ^^^^^^^^^^^^^^^^

warning: missing documentation for a function
  --> submodules/parking_lot/core/src/parking_lot/deadlock.rs:50:5
   |
50 |     pub unsafe fn on_unpark(_td: &crate::parking_lot::ThreadData) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `parking_lot_core` (lib) generated 19 warnings (run `cargo fix --lib -p parking_lot_core` to apply 11 suggestions)
warning: unused import: `deadlock`
  --> submodules/parking_lot/src/condvar.rs:10:13
   |
10 | use crate::{deadlock, util};
   |             ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `crate::deadlock`
  --> submodules/parking_lot/src/raw_mutex.rs:11:5
   |
11 | use crate::deadlock;
   |     ^^^^^^^^^^^^^^^

warning: unused import: `parking_lot_core::deadlock::deadlock as deadlock`
  --> submodules/parking_lot/src/deadlock.rs:38:16
   |
38 | pub(crate) use parking_lot_core::deadlock::deadlock as deadlock;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `parking_lot` (lib) generated 3 warnings (run `cargo fix --lib -p parking_lot` to apply 3 suggestions)
warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/rust-shlex/src/lib.rs:228:22
    |
228 | pub fn quote(in_str: &str) -> Cow<str> {
    |                      ^^^^     ^^^^^^^^ the same lifetime is hidden here
    |                      |
    |                      the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
    = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: use `'_` for type paths
    |
228 | pub fn quote(in_str: &str) -> Cow<'_, str> {
    |                                   +++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/rust-shlex/src/lib.rs:241:26
    |
241 | pub fn try_quote(in_str: &str) -> Result<Cow<str>, QuoteError> {
    |                          ^^^^            ^^^^^^^^ the same lifetime is hidden here
    |                          |
    |                          the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
241 | pub fn try_quote(in_str: &str) -> Result<Cow<'_, str>, QuoteError> {
    |                                              +++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/rust-shlex/src/bytes.rs:485:24
    |
485 | pub fn quote(in_bytes: &[u8]) -> Cow<[u8]> {
    |                        ^^^^^     ^^^^^^^^^ the same lifetime is hidden here
    |                        |
    |                        the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
485 | pub fn quote(in_bytes: &[u8]) -> Cow<'_, [u8]> {
    |                                      +++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/rust-shlex/src/bytes.rs:498:28
    |
498 | pub fn try_quote(in_bytes: &[u8]) -> Result<Cow<[u8]>, QuoteError> {
    |                            ^^^^^            ^^^^^^^^^ the same lifetime is hidden here
    |                            |
    |                            the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
498 | pub fn try_quote(in_bytes: &[u8]) -> Result<Cow<'_, [u8]>, QuoteError> {
    |                                                 +++

warning: `shlex` (lib) generated 4 warnings (run `cargo fix --lib -p shlex` to apply 4 suggestions)
warning: unexpected `cfg` condition value: `128`
   --> submodules/atomic-maybe-uninit/src/lib.rs:806:7
    |
806 | #[cfg(target_pointer_width = "128")]
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `target_pointer_width` are: `16`, `32`, and `64`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition name: `atomic_maybe_uninit_test_prefer_kuser_cmpxchg`
   --> submodules/atomic-maybe-uninit/src/lib.rs:825:13
    |
825 |             atomic_maybe_uninit_test_prefer_kuser_cmpxchg,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: expected names are: `atomic_maybe_uninit_no_asm`, `atomic_maybe_uninit_no_cmpxchg`, `atomic_maybe_uninit_no_cmpxchg8b`, `atomic_maybe_uninit_no_const_mut_refs`, `atomic_maybe_uninit_no_diagnostic_namespace`, `atomic_maybe_uninit_no_ldex_stex`, `atomic_maybe_uninit_no_strict_provenance`, `atomic_maybe_uninit_no_sync`, `atomic_maybe_uninit_pre_llvm_20`, `atomic_maybe_uninit_target_feature`, `atomic_maybe_uninit_unstable_asm_experimental_arch`, `docsrs`, `feature`, and `test` and 31 more
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(atomic_maybe_uninit_test_prefer_kuser_cmpxchg)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(atomic_maybe_uninit_test_prefer_kuser_cmpxchg)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `atomic_maybe_uninit_test_prefer_kuser_cmpxchg`
   --> submodules/atomic-maybe-uninit/src/lib.rs:836:13
    |
836 |             atomic_maybe_uninit_test_prefer_kuser_cmpxchg,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(atomic_maybe_uninit_test_prefer_kuser_cmpxchg)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(atomic_maybe_uninit_test_prefer_kuser_cmpxchg)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `128`
  --> submodules/atomic-maybe-uninit/src/gen/utils.rs:96:11
   |
96 |     #[cfg(target_pointer_width = "128")]
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `target_pointer_width` are: `16`, `32`, and `64`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `128`
   --> submodules/atomic-maybe-uninit/src/utils.rs:353:15
    |
353 |         #[cfg(target_pointer_width = "128")]
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: submodules/atomic-maybe-uninit/src/arch/aarch64.rs:34:1
    |
 34 | delegate_size!(delegate_all);
    | ---------------------------- in this macro invocation
    |
    = note: expected values for `target_pointer_width` are: `16`, `32`, and `64`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: this warning originates in the macro `delegate_size` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unexpected `cfg` condition value: `128`
   --> submodules/atomic-maybe-uninit/src/utils.rs:355:15
    |
355 |         #[cfg(target_pointer_width = "128")]
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: submodules/atomic-maybe-uninit/src/arch/aarch64.rs:34:1
    |
 34 | delegate_size!(delegate_all);
    | ---------------------------- in this macro invocation
    |
    = note: expected values for `target_pointer_width` are: `16`, `32`, and `64`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: this warning originates in the macro `delegate_size` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unexpected `cfg` condition value: `128`
   --> submodules/atomic-maybe-uninit/src/lib.rs:973:11
    |
973 |     #[cfg(target_pointer_width = "128")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `target_pointer_width` are: `16`, `32`, and `64`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: `atomic-maybe-uninit` (lib) generated 7 warnings
warning: unexpected `cfg` condition value: `arbitrary`
 --> submodules/dashmap/src/lib.rs:4:7
  |
4 | #[cfg(feature = "arbitrary")]
  |       ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: expected values for `feature` are: `all`, `inline-more`, `raw-api`, and `typesize`
  = help: consider adding `arbitrary` as a feature in `Cargo.toml`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value: `serde`
  --> submodules/dashmap/src/lib.rs:11:7
   |
11 | #[cfg(feature = "serde")]
   |       ^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `all`, `inline-more`, `raw-api`, and `typesize`
   = help: consider adding `serde` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
  --> submodules/dashmap/src/lib.rs:18:7
   |
18 | #[cfg(feature = "rayon")]
   |       ^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `all`, `inline-more`, `raw-api`, and `typesize`
   = help: consider adding `rayon` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: `dashmap` (lib) generated 3 warnings
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/adler32.c:8:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/compress.c:9:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/crc32.c:30:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/deflate.h:16,
warning: libz-sys@1.1.23:                  from src/zlib/deflate.c:52:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/infback.c:13:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/inffast.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/inflate.c:83:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/inftrees.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/deflate.h:16,
warning: libz-sys@1.1.23:                  from src/zlib/trees.c:37:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/uncompr.c:9:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/limits.h:26,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:210,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/syslimits.h:7,
warning: libz-sys@1.1.23:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/limits.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zconf.h:420,
warning: libz-sys@1.1.23:                  from src/zlib/zlib.h:34,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.h:22,
warning: libz-sys@1.1.23:                  from src/zlib/zutil.c:8:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libz-sys@1.1.23:                  from src/zlib/gzguts.h:20,
warning: libz-sys@1.1.23:                  from src/zlib/gzclose.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libz-sys@1.1.23:                  from src/zlib/gzguts.h:20,
warning: libz-sys@1.1.23:                  from src/zlib/gzlib.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libz-sys@1.1.23:                  from src/zlib/gzguts.h:20,
warning: libz-sys@1.1.23:                  from src/zlib/gzread.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: libz-sys@1.1.23: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libz-sys@1.1.23:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libz-sys@1.1.23:                  from src/zlib/gzguts.h:20,
warning: libz-sys@1.1.23:                  from src/zlib/gzwrite.c:6:
warning: libz-sys@1.1.23: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libz-sys@1.1.23:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libz-sys@1.1.23:       |    ^~~~~~~
warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/shell-words/src/lib.rs:282:17
    |
282 | pub fn quote(s: &str) -> Cow<str> {
    |                 ^^^^     ^^^^^^^^ the same lifetime is hidden here
    |                 |
    |                 the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
    = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: use `'_` for type paths
    |
282 | pub fn quote(s: &str) -> Cow<'_, str> {
    |                              +++

warning: `shell-words` (lib) generated 1 warning (run `cargo fix --lib -p shell-words` to apply 1 suggestion)
warning: unexpected `cfg` condition value: `bitrig`
  --> submodules/filetime/src/unix/mod.rs:88:11
   |
88 |     #[cfg(target_os = "bitrig")]
   |           ^^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `target_os` are: `aix`, `amdhsa`, `android`, `cuda`, `cygwin`, `dragonfly`, `emscripten`, `espidf`, `freebsd`, `fuchsia`, `haiku`, `hermit`, `horizon`, `hurd`, `illumos`, `ios`, `l4re`, `linux`, `lynxos178`, `macos`, `managarm`, `netbsd`, `none`, `nto`, `nuttx`, `openbsd`, `psp`, `psx`, `redox`, `rtems`, `solaris`, `solid_asp3`, `teeos`, `trusty`, and `tvos` and 11 more
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value: `bitrig`
  --> submodules/filetime/src/unix/mod.rs:97:15
   |
97 |     #[cfg(not(target_os = "bitrig"))]
   |               ^^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `target_os` are: `aix`, `amdhsa`, `android`, `cuda`, `cygwin`, `dragonfly`, `emscripten`, `espidf`, `freebsd`, `fuchsia`, `haiku`, `hermit`, `horizon`, `hurd`, `illumos`, `ios`, `l4re`, `linux`, `lynxos178`, `macos`, `managarm`, `netbsd`, `none`, `nto`, `nuttx`, `openbsd`, `psp`, `psx`, `redox`, `rtems`, `solaris`, `solid_asp3`, `teeos`, `trusty`, and `tvos` and 11 more
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `emulate_second_only_system`
  --> submodules/filetime/src/lib.rs:82:17
   |
82 |         if cfg!(emulate_second_only_system) {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: expected names are: `docsrs`, `feature`, and `test` and 31 more
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(emulate_second_only_system)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(emulate_second_only_system)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: `filetime` (lib) generated 3 warnings
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/altsvc.c:28:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/asyn-base.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/asyn-thrdd.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/bufq.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/bufref.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-h1-proxy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-haproxy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-https-connect.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-ip-happy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-socket.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cfilters.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/conncache.c:26:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/connect.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/content_encoding.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cookie.c:72:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cshutdn.c:26:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_addrinfo.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_fopen.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_get_line.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_memrchr.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_range.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_sha512_256.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_threads.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_trc.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/base64.c:27:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/dynbuf.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/fopen.c:33:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/inet_pton.c:21:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/inet_ntop.c:20:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/nonblock.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/strerr.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/strparse.h:26,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/strparse.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/timediff.h:27,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/timediff.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/timeval.h:27,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/timeval.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/wait.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/warnless.h:27,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curlx/warnless.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cw-out.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cw-pause.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/doh.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/dynhds.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/easy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/escape.c:28:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/file.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/fileinfo.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/formdata.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/getenv.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/getinfo.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/hash.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/headers.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/hostip.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/hmac.c:27:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/hostip6.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/hsts.c:28:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http1.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http_aws_sigv4.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http_chunks.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http_digest.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http_proxy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/idn.c:29:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/if2ip.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/llist.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/macos.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/md5.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/mime.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/mprintf.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/mqtt.c:26:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/multi.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/multi_ev.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/multi_ntfy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/netrc.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/noproxy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/parsedate.c:78:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/progress.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/rand.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/rename.c:27:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/request.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/select.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/sendf.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/setopt.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/sha256.c:26:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/share.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/slist.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/socketpair.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/socks.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/speedcheck.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/splay.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/strcase.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/strdup.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/strerror.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/strequal.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/transfer.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/uint-bset.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/uint-hash.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/uint-spbset.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/uint-table.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/url.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/urlapi.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vauth/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vauth/digest.c:28:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vauth/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vauth/vauth.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/version.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/curl_ngtcp2.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/curl_osslq.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/curl_quiche.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/vquic-tls.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vquic/vquic.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/hostcheck.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/keylog.c:24:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/vtls.c:41:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/vtls_scache.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/ws.c:24:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/cf-h2-proxy.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/http2.c:25:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: curl-sys@0.4.84+curl-8.17.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/pthread.h:21,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/../curl_setup.h:223,
warning: curl-sys@0.4.84+curl-8.17.0:                  from curl/lib/vtls/openssl.c:30:
warning: curl-sys@0.4.84+curl-8.17.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: curl-sys@0.4.84+curl-8.17.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: curl-sys@0.4.84+curl-8.17.0:       |    ^~~~~~~
warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/encoding_rs/src/handles.rs:390:21
    |
390 |     pub fn new(src: &[u8]) -> ByteSource {
    |                     ^^^^^     ^^^^^^^^^^ the same lifetime is hidden here
    |                     |
    |                     the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
    = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: use `'_` for type paths
    |
390 |     pub fn new(src: &[u8]) -> ByteSource<'_> {
    |                                         ++++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/encoding_rs/src/handles.rs:597:21
    |
597 |     pub fn new(dst: &mut [u16]) -> Utf16Destination {
    |                     ^^^^^^^^^^     ^^^^^^^^^^^^^^^^ the same lifetime is hidden here
    |                     |
    |                     the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
597 |     pub fn new(dst: &mut [u16]) -> Utf16Destination<'_> {
    |                                                    ++++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/encoding_rs/src/handles.rs:942:21
    |
942 |     pub fn new(dst: &mut [u8]) -> Utf8Destination {
    |                     ^^^^^^^^^     ^^^^^^^^^^^^^^^ the same lifetime is hidden here
    |                     |
    |                     the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
942 |     pub fn new(dst: &mut [u8]) -> Utf8Destination<'_> {
    |                                                  ++++

warning: hiding a lifetime that's elided elsewhere is confusing
    --> submodules/encoding_rs/src/handles.rs:1167:21
     |
1167 |     pub fn new(src: &[u16]) -> Utf16Source {
     |                     ^^^^^^     ^^^^^^^^^^^ the same lifetime is hidden here
     |                     |
     |                     the lifetime is elided here
     |
     = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
     |
1167 |     pub fn new(src: &[u16]) -> Utf16Source<'_> {
     |                                           ++++

warning: hiding a lifetime that's elided elsewhere is confusing
    --> submodules/encoding_rs/src/handles.rs:1469:21
     |
1469 |     pub fn new(src: &str) -> Utf8Source {
     |                     ^^^^     ^^^^^^^^^^ the same lifetime is hidden here
     |                     |
     |                     the lifetime is elided here
     |
     = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
     |
1469 |     pub fn new(src: &str) -> Utf8Source<'_> {
     |                                        ++++

warning: hiding a lifetime that's elided elsewhere is confusing
    --> submodules/encoding_rs/src/handles.rs:1925:21
     |
1925 |     pub fn new(dst: &mut [u8]) -> ByteDestination {
     |                     ^^^^^^^^^     ^^^^^^^^^^^^^^^ the same lifetime is hidden here
     |                     |
     |                     the lifetime is elided here
     |
     = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
     |
1925 |     pub fn new(dst: &mut [u8]) -> ByteDestination<'_> {
     |                                                  ++++

warning: `encoding_rs` (lib) generated 6 warnings (run `cargo fix --lib -p encoding_rs` to apply 6 suggestions)
warning: libgit2-sys@0.18.2+1.9.1: failed to probe system libgit2:
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/annotated_commit.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/annotated_commit.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/apply.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/apply.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attr.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attr.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attr_file.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attr_file.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attrcache.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/attrcache.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blame.h:4,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blame.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blame_git.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blame_git.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blob.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/blob.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/branch.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/branch.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/buffer.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/buf.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/buf.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/cache.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/cache.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/checkout.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/checkout.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/cherrypick.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/clone.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/clone.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit_graph.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit_graph.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit_list.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/commit_list.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_cache.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_file.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_list.h:8,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_list.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_mem.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_parse.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_parse.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_backend.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/config_snapshot.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/crlf.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/delta.h:8,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/delta.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/describe.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_driver.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_driver.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_file.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_file.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_generate.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_generate.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_parse.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_parse.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_print.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_stats.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_stats.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_tform.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_tform.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_xdiff.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/diff_xdiff.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/email.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/email.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/fetch.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/fetch.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/fetchhead.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/fetchhead.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/filter.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/filter.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/grafts.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/grafts.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/graph.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/hashsig.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/ident.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/ignore.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/ignore.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/index.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/index.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/index_map.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/iterator.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/iterator.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/indexer.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/indexer.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/annotated_commit.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/libgit2.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/mailmap.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/mailmap.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/mailmap.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/merge.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/merge.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/merge_driver.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/merge_driver.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/merge_file.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/buffer.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/buf.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/message.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/midx.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/midx.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/mwindow.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/mwindow.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/notes.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/notes.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/object.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/object.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/object_api.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/odb.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/odb.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/odb_loose.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/odb_mempack.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/odb_pack.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/oid.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/oid.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/oidarray.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/oidarray.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pack.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pack.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/parse.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/parse.c:7:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pack-objects.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pack-objects.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch_generate.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch_generate.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch_parse.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/patch_parse.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/path.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/path.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pathspec.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/pathspec.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/proxy.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/proxy.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/push.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/push.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/reader.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/reader.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/rebase.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refdb.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refdb.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refs.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refdb_fs.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/reflog.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/reflog.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refs.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refs.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/revert.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/remote.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/remote.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/reset.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refspec.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/refspec.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/repository.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/repository.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/revparse.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/revwalk.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/revwalk.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/annotated_commit.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/settings.c:10:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/signature.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/signature.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/stash.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/status.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/status.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/str.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/util.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/strarray.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/submodule.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/submodule.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/sysdir.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/sysdir.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tag.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tag.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/array.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/trailer.c:7:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/trace.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/trace.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transaction.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transaction.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transport.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tree-cache.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tree-cache.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tree.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/tree.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/worktree.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/worktree.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/sys/alloc.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/alloc.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/alloc.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/date.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/errors.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/filebuf.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/filebuf.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/fs_path.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/fs_path.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/futils.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/futils.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/net.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/net.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/pool.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/pool.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/posix.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/posix.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/pqueue.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/pqueue.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/rand.c:9:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/runtime.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/sortedcache.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/sortedcache.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/str.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/str.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/strlist.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/thread.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/tsort.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/str.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/util.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/util.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/varint.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/varint.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/utf8.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/utf8.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/vector.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/vector.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/wildmatch.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/wildmatch.c:19:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/zstream.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/zstream.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_negotiate.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_gssapi.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_ntlm.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_ntlmclient.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_ntlm.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/auth_sspi.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/credential.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/credential_helpers.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/git.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/http.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/httpparser.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/httpparser.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/httpclient.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/local.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/smart.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/smart.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/smart_pkt.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/smart_protocol.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh_exec.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh_exec.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh_exec.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh_libssh2.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/ssh_libssh2.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/transports/winhttp.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/mbedtls.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/mbedtls.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl_dynamic.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/openssl_legacy.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/registry.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/schannel.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/schannel.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/socket.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/socket.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/errors.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/tls.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/stransport.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/streams/stransport.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdlib.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/llhttp/llhttp.c:1:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdlib.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/llhttp/api.c:1:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/llhttp/http.c:1:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xdiffi.c:23:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xemit.c:23:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xhistogram.c:44:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xmerge.c:23:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xpatience.c:22:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xprepare.c:23:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/regexp.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/git-xdiff.h:17,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xinclude.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/xdiff/xutils.c:23:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_byte_order.c:50:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_chartables.c:27:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_compile.c:53:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_config.c:51:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_dfa_exec.c:83:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_exec.c:52:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_fullinfo.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_get.c:50:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_globals.c:59:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_jit_compile.c:47:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_maketables.c:51:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_newline.c:54:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_ord2utf8.c:50:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdlib.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre.h:54,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_printint.c:70:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_refcount.c:51:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_string_utils.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_tables.c:52:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_study.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_ucd.c:17:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_valid_utf8.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_version.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/ctype.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_internal.h:108,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre_xclass.c:49:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdlib.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcre.h:54,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/deps/pcre/pcreposix.c:54:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/allocators/failalloc.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/allocators/failalloc.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/allocators/stdalloc.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/allocators/stdalloc.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/unix/map.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/unix/process.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/unix/realpath.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/sha.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/collisiondetect.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/collisiondetect.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/sha1dc/sha1.c:17:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/libgit2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/sha1dc/ubc_check.c:31:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/time.h:25,
warning: libgit2-sys@0.18.2+1.9.1:                  from /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/target/debug/build/libgit2-sys-1afe4bb4a81d3198/out/include/git2/common.h:10,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/git2_util.h:14,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/sha.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/builtin.h:11,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/builtin.c:8:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: libgit2-sys@0.18.2+1.9.1: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdint.h:26,
warning: libgit2-sys@0.18.2+1.9.1:                  from /nix/store/49spybp2k23p892159fi937x1axk9gvz-gcc-14.3.0/lib/gcc/aarch64-unknown-linux-gnu/14.3.0/include/stdint.h:9,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/rfc6234/sha.h:73,
warning: libgit2-sys@0.18.2+1.9.1:                  from libgit2/src/util/hash/rfc6234/sha224-256.c:43:
warning: libgit2-sys@0.18.2+1.9.1: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libgit2-sys@0.18.2+1.9.1:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libgit2-sys@0.18.2+1.9.1:       |    ^~~~~~~
warning: unused import: `hash::*`
  --> submodules/simd-adler32/src/lib.rs:98:9
   |
98 | pub use hash::*;
   |         ^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `simd-adler32` (lib) generated 1 warning (run `cargo fix --lib -p simd-adler32` to apply 1 suggestion)
warning: unexpected `cfg` condition value: `borsh`
   --> submodules/indexmap/src/lib.rs:111:7
    |
111 | #[cfg(feature = "borsh")]
    |       ^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `borsh` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition value: `sval`
   --> submodules/indexmap/src/lib.rs:115:7
    |
115 | #[cfg(feature = "sval")]
    |       ^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `sval` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
   --> submodules/indexmap/src/lib.rs:124:7
    |
124 | #[cfg(feature = "rayon")]
    |       ^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `rayon` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `arbitrary`
 --> submodules/indexmap/src/arbitrary.rs:1:7
  |
1 | #[cfg(feature = "arbitrary")]
  |       ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
  = help: consider adding `arbitrary` as a feature in `Cargo.toml`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `quickcheck`
  --> submodules/indexmap/src/arbitrary.rs:38:7
   |
38 | #[cfg(feature = "quickcheck")]
   |       ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
   = help: consider adding `quickcheck` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
   --> submodules/indexmap/src/macros.rs:202:7
    |
202 | #[cfg(feature = "rayon")]
    |       ^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `rayon` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
   --> submodules/indexmap/src/macros.rs:227:7
    |
227 | #[cfg(feature = "rayon")]
    |       ^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `rayon` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
  --> submodules/indexmap/src/map.rs:26:7
   |
26 | #[cfg(feature = "rayon")]
   |       ^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
   = help: consider adding `rayon` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
   --> submodules/indexmap/src/map/core.rs:193:11
    |
193 |     #[cfg(feature = "rayon")]
    |           ^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
    = help: consider adding `rayon` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition value: `rayon`
  --> submodules/indexmap/src/set.rs:16:7
   |
16 | #[cfg(feature = "rayon")]
   |       ^^^^^^^^^^^^^^^^^
   |
   = note: expected values for `feature` are: `default`, `serde`, `std`, and `test_debug`
   = help: consider adding `rayon` as a feature in `Cargo.toml`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: use of deprecated method `hashbrown::HashTable::<T, A>::get_many_mut`: use `get_disjoint_mut` instead
   --> submodules/indexmap/src/map/core.rs:744:28
    |
744 |         match self.indices.get_many_mut(
    |                            ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: `indexmap` (lib) generated 11 warnings
warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/minimal-lexical/src/bigint.rs:127:17
    |
127 | pub fn rview(x: &[Limb]) -> ReverseView<Limb> {
    |                 ^^^^^^^     ^^^^^^^^^^^^^^^^^ the same lifetime is hidden here
    |                 |
    |                 the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
    = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: use `'_` for type paths
    |
127 | pub fn rview(x: &[Limb]) -> ReverseView<'_, Limb> {
    |                                         +++

warning: `minimal-lexical` (lib) generated 1 warning (run `cargo fix --lib -p minimal-lexical` to apply 1 suggestion)
warning: libsqlite3-sys@0.35.0: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/bits/libc-header-start.h:33,
warning: libsqlite3-sys@0.35.0:                  from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/stdio.h:28,
warning: libsqlite3-sys@0.35.0:                  from sqlite3/sqlite3.c:15244:
warning: libsqlite3-sys@0.35.0: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: libsqlite3-sys@0.35.0:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: libsqlite3-sys@0.35.0:       |    ^~~~~~~
   Compiling im-rc v15.1.0 (/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/im-rs/rc)
warning: blake3@1.8.2: In file included from /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/assert.h:35,
warning: blake3@1.8.2:                  from c/blake3_impl.h:4,
warning: blake3@1.8.2:                  from c/blake3_neon.c:1:
warning: blake3@1.8.2: /nix/store/21hfs53hjb8spgp3558dn7cr0phs38kf-glibc-2.40-66-dev/include/features.h:422:4: warning: #warning _FORTIFY_SOURCE requires compiling with optimization (-O) [-Wcpp]
warning: blake3@1.8.2:   422 | #  warning _FORTIFY_SOURCE requires compiling with optimization (-O)
warning: blake3@1.8.2:       |    ^~~~~~~
warning: associated function `reject_noncanonical` is never used
   --> submodules/rust-ed25519-compact/src/field25519.rs:677:12
    |
524 | impl Fe {
    | ------- associated function in this implementation
...
677 |     pub fn reject_noncanonical(s: &[u8]) -> Result<(), Error> {
    |            ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ed25519-compact` (lib) generated 1 warning
warning: `...` range patterns are deprecated
  --> submodules/shell-escape/src/lib.rs:95:16
   |
95 |             'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' | '=' | '/' | ',' | '.' | '+' => false,
   |                ^^^ help: use `..=` for an inclusive range
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: `#[warn(ellipsis_inclusive_range_patterns)]` (part of `#[warn(rust_2021_compatibility)]`) on by default

warning: `...` range patterns are deprecated
  --> submodules/shell-escape/src/lib.rs:95:28
   |
95 |             'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' | '=' | '/' | ',' | '.' | '+' => false,
   |                            ^^^ help: use `..=` for an inclusive range
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2021/warnings-promoted-to-error.html>

warning: `...` range patterns are deprecated
  --> submodules/shell-escape/src/lib.rs:95:40
   |
95 |             'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' | '=' | '/' | ',' | '.' | '+' => false,
   |                                        ^^^ help: use `..=` for an inclusive range
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2021/warnings-promoted-to-error.html>

warning: `shell-escape` (lib) generated 3 warnings (run `cargo fix --lib -p shell-escape` to apply 3 suggestions)
warning: use of deprecated method `p384::elliptic_curve::generic_array::GenericArray::<T, N>::as_slice`: please upgrade to generic-array 1.x
  --> submodules/pasetors/src/version3.rs:92:69
   |
92 |         let secret = AsymmetricSecretKey::<V3>::from(key.to_bytes().as_slice())?;
   |                                                                     ^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: `pasetors` (lib) generated 1 warning
warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/unicode-segmentation/src/lib.rs:261:25
    |
261 |     fn grapheme_indices(&self, is_extended: bool) -> GraphemeIndices {
    |                         ^^^^^                        ^^^^^^^^^^^^^^^ the same lifetime is hidden here
    |                         |
    |                         the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
    = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: use `'_` for type paths
    |
261 |     fn grapheme_indices(&self, is_extended: bool) -> GraphemeIndices<'_> {
    |                                                                     ++++

warning: hiding a lifetime that's elided elsewhere is confusing
   --> submodules/unicode-segmentation/src/lib.rs:296:37
    |
296 |     fn split_sentence_bound_indices(&self) -> USentenceBoundIndices {
    |                                     ^^^^^     ^^^^^^^^^^^^^^^^^^^^^ the same lifetime is hidden here
    |                                     |
    |                                     the lifetime is elided here
    |
    = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
    |
296 |     fn split_sentence_bound_indices(&self) -> USentenceBoundIndices<'_> {
    |                                                                    ++++

warning: `unicode-segmentation` (lib) generated 2 warnings (run `cargo fix --lib -p unicode-segmentation` to apply 2 suggestions)
warning: use of deprecated associated function `chrono::NaiveDateTime::from_timestamp_opt`: use `DateTime::from_timestamp` instead
   --> submodules/tera/src/builtins/filters/common.rs:175:43
    |
175 |                 let date = NaiveDateTime::from_timestamp_opt(i, 0).expect(
    |                                           ^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: eliding a lifetime that's named elsewhere is confusing
  --> submodules/lazy-static/src/inline_lazy.rs:26:43
   |
26 |     pub fn get<F>(&'static self, f: F) -> &T
   |                    -------                ^^ the same lifetime is elided here
   |                    |
   |                    the lifetime is named here
   |
   = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
   = note: `#[warn(mismatched_lifetime_syntaxes)]` on by default
help: consistently use `'static`
   |
26 |     pub fn get<F>(&'static self, f: F) -> &'static T
   |                                            +++++++

warning: `tera` (lib) generated 1 warning
warning: `lazy_static` (lib) generated 1 warning (run `cargo fix --lib -p lazy_static` to apply 1 suggestion)
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   --> submodules/im-rs/rc/../src/lib.rs:341:41
    |
341 | #![cfg_attr(has_specialisation, feature(specialization))]
    |                                         ^^^^^^^^^^^^^^
    |
    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
    = help: consider using `min_specialization` instead, which is more stable and complete
    = note: `#[warn(incomplete_features)]` on by default

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/lib.rs:341:13
    |
341 | #![cfg_attr(has_specialisation, feature(specialization))]
    |             ^^^^^^^^^^^^^^^^^^
    |
    = help: expected names are: `docsrs`, `feature`, and `test` and 31 more
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: `#[warn(unexpected_cfgs)]` on by default

warning: unexpected `cfg` condition name: `threadsafe`
   --> submodules/im-rs/rc/../src/lib.rs:381:11
    |
381 | #[cfg(all(threadsafe, feature = "quickcheck"))]
    |           ^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
   --> submodules/im-rs/rc/../src/lib.rs:385:11
    |
385 | #[cfg(any(threadsafe, not(feature = "pool")))]
    |           ^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
   --> submodules/im-rs/rc/../src/lib.rs:388:11
    |
388 | #[cfg(all(threadsafe, feature = "pool"))]
    |           ^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
 --> submodules/im-rs/rc/../src/sync.rs:7:7
  |
7 | #[cfg(threadsafe)]
  |       ^^^^^^^^^^
  |
  = help: consider using a Cargo feature instead
  = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
           [lints.rust]
           unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
  = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/sync.rs:38:11
   |
38 | #[cfg(not(threadsafe))]
   |           ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:17:11
   |
17 | #[cfg(all(threadsafe))]
   |           ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:21:7
   |
21 | #[cfg(threadsafe)]
   |       ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:25:15
   |
25 | #[cfg(all(not(threadsafe), not(feature = "pool")))]
   |               ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:29:15
   |
29 | #[cfg(all(not(threadsafe), feature = "pool"))]
   |               ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:31:15
   |
31 | #[cfg(all(not(threadsafe), feature = "pool"))]
   |               ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/util.rs:35:11
   |
35 | #[cfg(not(threadsafe))]
   |           ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
  --> submodules/im-rs/rc/../src/ord/map.rs:31:7
   |
31 | #[cfg(has_specialisation)]
   |       ^^^^^^^^^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
  --> submodules/im-rs/rc/../src/ord/map.rs:70:11
   |
70 | #[cfg(not(has_specialisation))]
   |           ^^^^^^^^^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/ord/map.rs:103:7
    |
103 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/ord/map.rs:136:7
    |
136 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/ord/map.rs:1676:11
     |
1676 | #[cfg(not(has_specialisation))]
     |           ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/ord/map.rs:1687:7
     |
1687 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/ord/map.rs:1698:7
     |
1698 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
  --> submodules/im-rs/rc/../src/ord/set.rs:33:7
   |
33 | #[cfg(has_specialisation)]
   |       ^^^^^^^^^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
  --> submodules/im-rs/rc/../src/ord/set.rs:78:11
   |
78 | #[cfg(not(has_specialisation))]
   |           ^^^^^^^^^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/ord/set.rs:111:7
    |
111 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/ord/set.rs:144:7
    |
144 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1564:11
     |
1564 | #[cfg(not(has_specialisation))]
     |           ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1576:7
     |
1576 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1588:7
     |
1588 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1760:11
     |
1760 | #[cfg(not(has_specialisation))]
     |           ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1776:7
     |
1776 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/hash/map.rs:1792:7
     |
1792 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/hash/set.rs:806:11
    |
806 | #[cfg(not(has_specialisation))]
    |           ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/hash/set.rs:817:7
    |
817 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
   --> submodules/im-rs/rc/../src/hash/set.rs:828:7
    |
828 | #[cfg(has_specialisation)]
    |       ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
  --> submodules/im-rs/rc/../src/vector/mod.rs:71:11
   |
71 | #[cfg(all(threadsafe, any(test, feature = "rayon")))]
   |           ^^^^^^^^^^
   |
   = help: consider using a Cargo feature instead
   = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
            [lints.rust]
            unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
   = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
   = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/vector/mod.rs:1715:11
     |
1715 | #[cfg(not(has_specialisation))]
     |           ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/vector/mod.rs:1722:7
     |
1722 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `has_specialisation`
    --> submodules/im-rs/rc/../src/vector/mod.rs:1729:7
     |
1729 | #[cfg(has_specialisation)]
     |       ^^^^^^^^^^^^^^^^^^
     |
     = help: consider using a Cargo feature instead
     = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
              [lints.rust]
              unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_specialisation)'] }
     = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(has_specialisation)");` to the top of the `build.rs`
     = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
   --> submodules/im-rs/rc/../src/vector/focus.rs:280:7
    |
280 | #[cfg(threadsafe)]
    |       ^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

warning: unexpected `cfg` condition name: `threadsafe`
   --> submodules/im-rs/rc/../src/vector/focus.rs:283:7
    |
283 | #[cfg(threadsafe)]
    |       ^^^^^^^^^^
    |
    = help: consider using a Cargo feature instead
    = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
             [lints.rust]
             unexpected_cfgs = { level = "warn", check-cfg = ['cfg(threadsafe)'] }
    = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(threadsafe)");` to the top of the `build.rs`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration

   Compiling cargo v0.94.0 (/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/submodules/cargo)
warning: `im-rc` (lib) generated 39 warnings
warning: method `get_package_paths_for_nodes` is never used
   --> submodules/cargo/src/cargo/ops/tree/graph.rs:361:12
    |
167 | impl<'a> Graph<'a> {
    | ------------------ method in this implementation
...
361 |     pub fn get_package_paths_for_nodes(
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

   Compiling cargo2nix v0.1.0 (/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/crates/cargo2nix)
   Compiling cargo-repo-sync-lib v0.1.0 (/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tools/cargo-repo-sync-lib)
warning: `cargo` (lib) generated 1 warning
error[E0422]: cannot find struct, variant or union type `RepoSyncConfig` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:43:19
   |
43 |     let _config = RepoSyncConfig {
   |                   ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct through its public re-export
   |
 1 + use crate::RepoSyncConfig;
   |

error[E0422]: cannot find struct, variant or union type `RepoSyncConfig` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:31:18
   |
31 |     let config = RepoSyncConfig {
   |                  ^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct through its public re-export
   |
 1 + use crate::RepoSyncConfig;
   |

error[E0425]: cannot find function `run_submodule_status` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:42:5
   |
42 |     run_submodule_status(config)
   |     ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::run_submodule_status;
   |

error[E0433]: failed to resolve: use of undeclared type `RealFileSystemStat`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:34:33
   |
34 |     let real_file_system_stat = RealFileSystemStat::new(repo.clone());
   |                                 ^^^^^^^^^^^^^^^^^^ use of undeclared type `RealFileSystemStat`
   |
help: consider importing this struct
   |
 1 + use crate::fs_cache::RealFileSystemStat;
   |

error[E0405]: cannot find trait `FileSystemWriter` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:36:37
   |
36 |     let file_system_writer: Box<dyn FileSystemWriter> = if cli.dry_run {
   |                                     ^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this trait
   |
 1 + use crate::fs_writer::FileSystemWriter;
   |

error[E0433]: failed to resolve: use of undeclared type `CachedFileSystemWriter`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:37:18
   |
37 | ...   Box::new(CachedFileSystemWriter::new(Arc::new(real_file_system_stat.clone()), rollup_lock_arc.clone(), r...
   |                ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `CachedFileSystemWriter`
   |
help: consider importing this struct
   |
 1 + use crate::fs_writer::CachedFileSystemWriter;
   |

error[E0425]: cannot find value `RealFileSystemWriter` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:39:18
   |
39 |         Box::new(RealFileSystemWriter)
   |                  ^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this unit struct
   |
 1 + use crate::fs_writer::RealFileSystemWriter;
   |

error[E0425]: cannot find value `RealDepGraphProcessor` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:45:31
   |
45 |     let dep_graph_processor = RealDepGraphProcessor;
   |                               ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this unit struct through its public re-export
   |
 1 + use crate::RealDepGraphProcessor;
   |

error[E0425]: cannot find value `RealNonVendoredModuleFinder` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:54:31
   |
54 |     let non_vendored_finder = RealNonVendoredModuleFinder;
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this unit struct
   |
 1 + use crate::non_vendored_module_finder::RealNonVendoredModuleFinder;
   |

error[E0425]: cannot find value `RealDepGraphDataMerger` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:62:23
   |
62 |     let data_merger = RealDepGraphDataMerger;
   |                       ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this unit struct
   |
 1 + use crate::dep_graph_data_merger::RealDepGraphDataMerger;
   |

error[E0412]: cannot find type `MergedCrateInfo` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:65:48
   |
65 |     let mut sorted_merged_data: Vec<(&String, &MergedCrateInfo)> = merged_data.iter().collect();
   |                                                ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
 1 + use crate::dep_graph_data_merger::MergedCrateInfo;
   |

error[E0425]: cannot find value `RealLayer0Analyzer` in this scope
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:73:27
   |
73 |     let layer0_analyzer = RealLayer0Analyzer;
   |                           ^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this unit struct
   |
 1 + use crate::layer0_analyzer::RealLayer0Analyzer;
   |

error[E0433]: failed to resolve: use of undeclared type `RealCargoConfigPatcher`
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:82:32
   |
82 |     let cargo_config_patcher = RealCargoConfigPatcher::new(metadata_provider.clone());
   |                                ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `RealCargoConfigPatcher`
   |
help: consider importing this struct
   |
 1 + use crate::cargo_config_patcher::RealCargoConfigPatcher;
   |

error[E0425]: cannot find value `RealSubmoduleConfigPatcher` in this scope
   --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:100:36
    |
100 |     let submodule_config_patcher = RealSubmoduleConfigPatcher;
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
  1 + use crate::submodule_config_patcher::RealSubmoduleConfigPatcher;
    |

error[E0425]: cannot find value `RealWorkspaceRemover` in this scope
   --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:119:29
    |
119 |     let workspace_remover = RealWorkspaceRemover;
    |                             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
  1 + use crate::workspace_remover::RealWorkspaceRemover;
    |

warning: unused import: `regex::Regex`
 --> tools/cargo-repo-sync-lib/src/cargo_config_generator.rs:4:5
  |
4 | use regex::Regex;
  |     ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `Result`
 --> tools/cargo-repo-sync-lib/src/cargo_config_generator.rs:5:23
  |
5 | use anyhow::{Context, Result};
  |                       ^^^^^^

warning: unused import: `lazy_static::lazy_static`
 --> tools/cargo-repo-sync-lib/src/cargo_config_generator.rs:6:5
  |
6 | use lazy_static::lazy_static;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Document`
 --> tools/cargo-repo-sync-lib/src/cargo_config_generator.rs:7:17
  |
7 | use toml_edit::{Document, DocumentMut, Item, Table, Value};
  |                 ^^^^^^^^

warning: unused imports: `PathBuf` and `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^  ^^^^^^^

warning: unused import: `std::fs`
 --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused import: `FileSystemStat`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:11:43
   |
11 | use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
   |                                           ^^^^^^^^^^^^^^

warning: unused imports: `find_cargo_locks` and `find_cargo_manifests`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:13:28
   |
13 | use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
   |                            ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `cargo2nix::generate_cargo_nix::generate_cargo_nix`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:14:5
   |
14 | use cargo2nix::generate_cargo_nix::generate_cargo_nix;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `generate_patch_entries`, `parse_members_file`, and `update_config_toml`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:24:37
   |
24 | use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
   |                                     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused imports: `DryRunExecv`, `Execv`, `JsonCaptureExecv`, `ReportExecv`, and `SystemExecv`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:25:28
   |
25 | use crate::traits::execv::{Execv, SystemExecv, DryRunExecv, JsonCaptureExecv, ReportExecv};
   |                            ^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused imports: `GitExecutor`, `PureRustGitExecutor`, `SystemGhExecutor`, and `SystemGitExecutor`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:26:24
   |
26 | use crate::executors::{PureRustGitExecutor, SystemGitExecutor, GitExecutor, SystemGhExecutor};
   |                        ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `crate::analysis::workspace_remover::RealWorkspaceRemover`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:27:5
   |
27 | use crate::analysis::workspace_remover::RealWorkspaceRemover;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `CargoMetadataProvider` and `RealCargoMetadataProvider`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:28:48
   |
28 | use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
   |                                                ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `CargoTomlUpdater` and `RealCargoTomlUpdater`
  --> tools/cargo-repo-sync-lib/src/cli/commands/add_submodules.rs:29:43
   |
29 | use crate::analysis::cargo_toml_updater::{CargoTomlUpdater, RealCargoTomlUpdater};
   |                                           ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Context`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:1:22
  |
1 | use anyhow::{Result, Context};
  |                      ^^^^^^^

warning: unused imports: `Arc` and `Mutex`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:2:17
  |
2 | use std::sync::{Arc, Mutex};
  |                 ^^^  ^^^^^

warning: unused import: `git2::Repository`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:3:5
  |
3 | use git2::Repository;
  |     ^^^^^^^^^^^^^^^^

warning: unused imports: `PathBuf` and `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^  ^^^^^^^

warning: unused import: `std::fs`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused imports: `find_cargo_locks` and `find_cargo_manifests`
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:13:28
   |
13 | use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
   |                            ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `cargo2nix::generate_cargo_nix::generate_cargo_nix`
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:14:5
   |
14 | use cargo2nix::generate_cargo_nix::generate_cargo_nix;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `generate_patch_entries`, `parse_members_file`, and `update_config_toml`
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:23:37
   |
23 | use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
   |                                     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused imports: `DryRunExecv`, `Execv`, `JsonCaptureExecv`, `ReportExecv`, and `SystemExecv`
  --> tools/cargo-repo-sync-lib/src/cli/commands/submodule_status.rs:24:28
   |
24 | use crate::traits::execv::{Execv, SystemExecv, DryRunExecv, JsonCaptureExecv, ReportExecv};
   |                            ^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused imports: `generate_patch_entries`, `parse_members_file`, and `update_config_toml`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:23:37
   |
23 | use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
   |                                     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::analysis::workspace_remover::RealWorkspaceRemover`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:26:5
   |
26 | use crate::analysis::workspace_remover::RealWorkspaceRemover;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `CargoMetadataProvider` and `RealCargoMetadataProvider`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_nix.rs:27:48
   |
27 | use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
   |                                                ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `Arc` and `Mutex`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:2:17
  |
2 | use std::sync::{Arc, Mutex};
  |                 ^^^  ^^^^^

warning: unused import: `git2::Repository`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:3:5
  |
3 | use git2::Repository;
  |     ^^^^^^^^^^^^^^^^

warning: unused imports: `PathBuf` and `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^  ^^^^^^^

warning: unused import: `std::fs`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused imports: `RepoSyncConfig` and `run_submodule_status`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:9:13
  |
9 | use crate::{run_submodule_status, RepoSyncConfig};
  |             ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused imports: `FileSystemStat` and `RealFileSystemStat`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:11:23
   |
11 | use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
   |                       ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused imports: `find_cargo_locks` and `find_cargo_manifests`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:13:28
   |
13 | use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
   |                            ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `cargo2nix::generate_cargo_nix::generate_cargo_nix`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:14:5
   |
14 | use cargo2nix::generate_cargo_nix::generate_cargo_nix;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::analysis::workspace_remover::RealWorkspaceRemover`
  --> tools/cargo-repo-sync-lib/src/cli/commands/generate_patches.rs:25:5
   |
25 | use crate::analysis::workspace_remover::RealWorkspaceRemover;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Mutex`
 --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:2:22
  |
2 | use std::sync::{Arc, Mutex};
  |                      ^^^^^

warning: unused import: `git2::Repository`
 --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:3:5
  |
3 | use git2::Repository;
  |     ^^^^^^^^^^^^^^^^

warning: unused imports: `PathBuf` and `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^  ^^^^^^^

warning: unused import: `std::fs`
 --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused imports: `FileSystemStat` and `RealFileSystemStat`
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:11:23
   |
11 | use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
   |                       ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused imports: `find_cargo_locks` and `find_cargo_manifests`
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:13:28
   |
13 | use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
   |                            ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `cargo2nix::generate_cargo_nix::generate_cargo_nix`
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:14:5
   |
14 | use cargo2nix::generate_cargo_nix::generate_cargo_nix;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `generate_patch_entries`, `parse_members_file`, and `update_config_toml`
  --> tools/cargo-repo-sync-lib/src/cli/commands/analyze.rs:23:37
   |
23 | use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
   |                                     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused import: `Context`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_workspaces.rs:1:22
  |
1 | use anyhow::{Result, Context};
  |                      ^^^^^^^

warning: unused import: `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/generate_workspaces.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused imports: `Arc` and `Mutex`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:2:17
  |
2 | use std::sync::{Arc, Mutex};
  |                 ^^^  ^^^^^

warning: unused import: `git2::Repository`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:3:5
  |
3 | use git2::Repository;
  |     ^^^^^^^^^^^^^^^^

warning: unused imports: `PathBuf` and `Path`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:4:17
  |
4 | use std::path::{Path, PathBuf};
  |                 ^^^^  ^^^^^^^

warning: unused import: `std::fs`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `MetadataCommand`, `PackageId`, and `Package`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:6:22
  |
6 | use cargo_metadata::{MetadataCommand, Package, PackageId};
  |                      ^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^

warning: unused imports: `RepoSyncConfig` and `run_submodule_status`
 --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:9:13
  |
9 | use crate::{run_submodule_status, RepoSyncConfig};
  |             ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused imports: `CachedFileSystemWriter`, `FileSystemWriter`, and `RealFileSystemWriter`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:10:24
   |
10 | use crate::fs_writer::{CachedFileSystemWriter, RealFileSystemWriter, FileSystemWriter};
   |                        ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused imports: `FileSystemStat` and `RealFileSystemStat`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:11:23
   |
11 | use crate::fs_cache::{RealFileSystemStat, FileSystemStat};
   |                       ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused imports: `find_cargo_locks` and `find_cargo_manifests`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:13:28
   |
13 | use cargo2nix::discovery::{find_cargo_manifests, find_cargo_locks};
   |                            ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `cargo2nix::generate_cargo_nix::generate_cargo_nix`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:14:5
   |
14 | use cargo2nix::generate_cargo_nix::generate_cargo_nix;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `DepGraphProcessor` and `RealDepGraphProcessor`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:17:44
   |
17 | use crate::analysis::dep_graph_processor::{DepGraphProcessor, RealDepGraphProcessor};
   |                                            ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `NonVendoredModuleFinder` and `RealNonVendoredModuleFinder`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:18:51
   |
18 | use crate::analysis::non_vendored_module_finder::{NonVendoredModuleFinder, RealNonVendoredModuleFinder};
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `DepGraphDataMerger`, `MergedCrateInfo`, and `RealDepGraphDataMerger`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:19:46
   |
19 | use crate::analysis::dep_graph_data_merger::{DepGraphDataMerger, RealDepGraphDataMerger, MergedCrateInfo};
   |                                              ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^

warning: unused imports: `Layer0Analyzer` and `RealLayer0Analyzer`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:20:40
   |
20 | use crate::analysis::layer0_analyzer::{Layer0Analyzer, RealLayer0Analyzer};
   |                                        ^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused imports: `CargoConfigPatcher` and `RealCargoConfigPatcher`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:21:45
   |
21 | use crate::analysis::cargo_config_patcher::{CargoConfigPatcher, RealCargoConfigPatcher};
   |                                             ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `RealSubmoduleConfigPatcher` and `SubmoduleConfigPatcher`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:22:49
   |
22 | use crate::analysis::submodule_config_patcher::{SubmoduleConfigPatcher, RealSubmoduleConfigPatcher};
   |                                                 ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `generate_patch_entries`, `parse_members_file`, and `update_config_toml`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:23:37
   |
23 | use crate::cargo_config_generator::{parse_members_file, generate_patch_entries, update_config_toml};
   |                                     ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::analysis::workspace_remover::RealWorkspaceRemover`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:25:5
   |
25 | use crate::analysis::workspace_remover::RealWorkspaceRemover;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `CargoMetadataProvider` and `RealCargoMetadataProvider`
  --> tools/cargo-repo-sync-lib/src/cli/commands/update_cargo_toml.rs:26:48
   |
26 | use crate::analysis::cargo_metadata_provider::{CargoMetadataProvider, RealCargoMetadataProvider};
   |                                                ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::path::PathBuf`
 --> tools/cargo-repo-sync-lib/src/cli/commands/collect_repo_state.rs:2:5
  |
2 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `Context` and `Result`
 --> tools/cargo-repo-sync-lib/src/cli/run_commands.rs:1:14
  |
1 | use anyhow::{Result, Context};
  |              ^^^^^^  ^^^^^^^

Some errors have detailed explanations: E0405, E0412, E0422, E0425, E0433.
For more information about an error, try `rustc --explain E0405`.
warning: `cargo-repo-sync-lib` (lib) generated 73 warnings
error: could not compile `cargo-repo-sync-lib` (lib) due to 15 previous errors; 73 warnings emitted

Cargo exited abnormally with code 101 at Fri Nov 21 01:42:48
