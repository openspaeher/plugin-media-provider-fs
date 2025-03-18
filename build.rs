use semver::Version;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wit");
    spaeher_contracts::download_contracts_sync(
        PathBuf::from("./wit"),
        Version::new(0, 1, 0),
        Version::new(0, 1, 0),
        Version::new(0, 1, 0),
    ).expect("Failed to download contracts");
}