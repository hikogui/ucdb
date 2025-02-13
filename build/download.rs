

fn download_and_open_file(url: &str, path: &std::path::Path) -> Result<io::File, String> {
    if std::fs::exists(&path).unwrap_or(false) {
        // File already exists, no need to download.
        match io::File::open(&path) {
            Err(e) => return Err(format_err("Could not open file:  {}", &e)),
            Ok(f)  => return f,
        };
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

    let mut out_file = match std::fs::File::create_new(&path) {
        Err(e) => return Err(format!("Could not create file: {:?}: {}", &path, e)),
        Ok(x) => x,
    };

    if let Err(e) = std::io::copy(&mut body.as_bytes(), &mut out_file) {
        return Err(format!("Could not copy data to file: {:?}: {}", &path, e));
    }

    // Go to the first byte of the file, so that we can start reading from it.
    if let Err(e) = out_file.seek(SeekFrom::Start(0)) {
        return Err(format!("Could not seek file:  {:?}:  {}", &path, e));
    }

    return out_file;
}

