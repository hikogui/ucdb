

fn ucd_download_file(url: &str, path: &std::path::Path) -> Result<(), String> {
    if std::fs::exists(&path).unwrap_or(false) {
        // File already exists, no need to download.
        return Ok(());
    }

    if let Some(dir) = path.parent() {
        // Create the dir that the file is downloaded in.
        if !std::fs::exists(&dir).unwrap_or(false) {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                return Err(format!("Could not create directory hierarchy {:?}: {}", &dir, e));
            }
        }
    }

    let response = match reqwest::blocking::get(url) {
        Err(e) => return Err(format!("Could not request download: {}: {}", &url, e)),
        Ok(x) => x,
    };

    if !response.status().is_success() {
        return Err(format!("Could not download: {}: {}", &url, response.status()));
    }

    let body = match response.text() {
        Err(e) => return Err(format!("Could not get body when downloading: {}: {}", &url, e)),
        Ok(x) => x,
    };

    let mut out = match std::fs::File::create(&path) {
        Err(e) => return Err(format!("Could not create file: {:?}: {}", &path, e)),
        Ok(x) => x,
    };

    if let Err(e) = std::io::copy(&mut body.as_bytes(), &mut out) {
        return Err(format!("Could not copy data to file: {:?}: {}", &path, e));
    }

    return Ok(());
}

pub fn ucd_download(ucd_url: &str, ucd_version: &str) -> Result<(), String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::Path::new(&manifest_dir);

    let files = vec![
        "ucd/BidiBrackets.txt",
        "ucd/BidiCharacterTest.txt",
        "ucd/BidiMirroring.txt",
        "ucd/BidiTest.txt",
        "ucd/CompositionExclusions.txt",
        "ucd/EastAsianWidth.txt",
        "ucd/auxiliary/GraphemeBreakProperty.txt",
        "ucd/auxiliary/GraphemeBreakTest.txt",
        "ucd/auxiliary/LineBreakTest.txt",
        "ucd/LineBreak.txt",
        "ucd/NormalizationTest.txt",
        "ucd/PropList.txt",
        "ucd/Scripts.txt",
        "ucd/auxiliary/SentenceBreakProperty.txt",
        "ucd/auxiliary/SentenceBreakTest.txt",
        "ucd/UnicodeData.txt",
        "ucd/auxiliary/WordBreakProperty.txt",
        "ucd/auxiliary/WordBreakTest.txt",
        "ucd/emoji/emoji-data.txt",
    ];

    for &file in &files {
        let url = format!("{}/{}/{}", ucd_url, ucd_version, file);
        let path = manifest_dir.join("data").join(ucd_version).join(file);

        if let Err(e) = ucd_download_file(&url, &path) {
            return Err(e);
        }
    }

    return Ok(());
}

