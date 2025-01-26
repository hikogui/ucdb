
extern crate cargo_metadata;

mod ucd_download;
use ucd_download::*;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let db = cargo_metadata::MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&manifest_dir)
        .exec()
        .unwrap();

    let root = db.root_package().unwrap();
    let ucd_version = root.metadata["ucd"]["version"].as_str().unwrap();
    let ucd_url = root.metadata["ucd"]["url"].as_str().unwrap();

    if let Err(e) = ucd_download(ucd_url, ucd_version) {
        println!("cargo::error={}", e);
    }

}

