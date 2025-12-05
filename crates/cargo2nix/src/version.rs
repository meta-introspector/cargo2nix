use semver::Version;

pub fn version() -> Version {
    // Since `CARGO_PKG_VERSION` is provided by Cargo itself, which uses the same `semver` crate to
    // parse version strings, the `unwrap()` below should never fail.
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
