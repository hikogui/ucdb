
extern crate cargo_metadata;

mod build_src;

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
    let data_dir = std::path::Path::new(&manifest_dir).join("data");
    let code_dir = std::path::Path::new(&manifest_dir).join("src");

    if let Err(e) = build_src::build(ucd_url, ucd_version, &data_dir, &code_dir) {
        println!("cargo::error={}", e);
    }
}

