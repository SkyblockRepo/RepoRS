use std::{
    fs::{File, OpenOptions, create_dir_all, exists, metadata, rename},
    io::{self, Write},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://github.com/SkyblockRepo/Repo/archive/main.zip";

    let mut response = ureq::get(url).call()?;

    if !(exists("SkyblockRepo-main.zip")? && metadata("Repo-main").is_err()) {
        if response.status() == 200 {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open("SkyblockRepo-main.zip")?;

            let content = response.body_mut().read_to_vec()?;
            file.write_all(&content)?;

            unzip(file);
        } else {
            return Err(format!("Reqwest failed with status {}", response.status()).into());
        }
    } else {
        eprintln!(
            "SkyblockRepo-main.zip and SkyblockRepo-main/ directory are present, if you wish to refetch them, delete them."
        )
    }

    Ok(())
}

fn unzip(file: File) {
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                use std::fs::{Permissions, set_permissions};

                set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    rename(Path::new("Repo-main"), Path::new("SkyblockRepo")).unwrap();
}
