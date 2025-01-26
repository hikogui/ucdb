

extern crate reqwest;

pub fn ucd_download(ucd_url: &str, ucd_version: &str) -> Result<(), String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::Path::new(&manifest_dir);

    let files = vec![
        "ucd/BidiBrackets.txt",
        "ucd/BidiCharacterTest.txt",
    ];

    for &file in &files {
        let url = format!("{}/{}/{}", ucd_url, ucd_version, file);
        let file = manifest_dir.join("data").join(ucd_version).join(file);

        // Check if the file already exists, if it does don't download it.
        if std::fs::exists(&file).unwrap_or(false) {
            continue;
        }

        if let Some(dir) = file.parent() {
            // Create the dir that the file is downloaded in.
            if !std::fs::exists(&dir).unwrap_or(false) {
                if let Err(e) = std::fs::create_dir_all(&dir) {
                    return Err(format!("Could not create directory hierarchy {:?}: {}", &dir, e));
                }
            }
        }

        let response = match reqwest::blocking::get(&url) {
            Err(e) => return Err(format!("Could not request download: {}: {}", &url, e)),
            Ok(x) => x,
        };

        let body = match response.text() {
            Err(e) => return Err(format!("Could not get body when downloading: {}: {}", &url, e)),
            Ok(x) => x,
        };

        let mut out = match std::fs::File::create(&file) {
            Err(e) => return Err(format!("Could not create file: {:?}: {}", &file, e)),
            Ok(x) => x,
        };

        if let Err(e) = std::io::copy(&mut body.as_bytes(), &mut out) {
            return Err(format!("Could not copy data to file: {:?}: {}", &file, e));
        }
    }

    return Ok(());
}

