use std::fs::{File, OpenOptions, create_dir_all, exists, remove_dir_all, remove_file, rename};
use std::io::{self, Write};
use std::path::Path;

#[cfg(feature = "log")]
use log::{error, trace};
use pyo3::exceptions::{PyIOError, PyRuntimeError};
use pyo3::{Bound, PyAny, PyErr, PyResult, Python, pyfunction};

/// Downloads the github SkyblockRepo data and unzips
///
/// You can additonally remove the downloaded zip and only keep the extracted directory by passing in `true`
#[pyfunction(name = "download_repo")]
#[pyo3(signature=(delete_zip=true))]
pub fn download_zip_python(
	delete_zip: bool,
	py: Python,
) -> PyResult<Bound<PyAny>> {
	pyo3_async_runtimes::tokio::future_into_py(py, async move {
		download_zip(delete_zip).await?;
		Ok(())
	})
}

/// Downloads the github SkyblockRepo data and unzips
///
/// You can additonally remove the downloaded zip and only keep the extracted directory by passing in `true`
pub async fn download_zip(delete_zip: bool) -> PyResult<()> {
	let url = "https://github.com/SkyblockRepo/Repo/archive/main.zip";

	let response = reqwest::get(url)
		.await
		.map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

	if !exists("SkyblockRepo")? || (!exists("SkyblockRepo-main.zip")? && !exists("SkyblockRepo")?) {
		if response.status() == 200 {
			let mut file = OpenOptions::new()
				.read(true)
				.write(true)
				.create_new(true)
				.open("SkyblockRepo-main.zip")?;

			let content = response
				.bytes()
				.await
				.map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
			file.write_all(&content)?;

			unzip_repo(file)?;
		} else {
			return Err(PyErr::new::<PyRuntimeError, _>(format!(
				"Reqwest failed with status {}",
				response.status()
			)));
		}
	} else {
		#[cfg(feature = "log")]
		error!(
			"SkyblockRepo-main.zip and/or SkyblockRepo/ directory are present, if you wish to refetch them, delete them."
		);
		return Ok(());
	}

	if delete_zip {
		remove_file(Path::new("SkyblockRepo-main.zip"))?;
	}

	Ok(())
}

fn unzip_repo(file: File) -> PyResult<()> {
	let mut archive =
		zip::ZipArchive::new(file).map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;

	for i in 0..archive.len() {
		let mut file = archive
			.by_index(i)
			.map_err(|e| PyErr::new::<PyIOError, _>(e.to_string()))?;
		let outpath = match file.enclosed_name() {
			| Some(path) => path,
			| None => continue,
		};

		if file.is_dir() {
			#[cfg(feature = "log")]
			trace!("File {} extracted to \"{}\"", i, outpath.display());
			create_dir_all(&outpath)?;
		} else {
			#[cfg(feature = "log")]
			trace!(
				"File {} extracted to \"{}\" ({} bytes)",
				i,
				outpath.display(),
				file.size()
			);
			if let Some(p) = outpath.parent() {
				if !p.exists() {
					create_dir_all(p)?;
				}
			}
			let mut outfile = File::create(&outpath)?;
			io::copy(&mut file, &mut outfile)?;
		}

		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;

			if let Some(mode) = file.unix_mode() {
				use std::fs::{Permissions, set_permissions};

				set_permissions(&outpath, Permissions::from_mode(mode))?;
			}
		}
	}

	rename(Path::new("Repo-main"), Path::new("SkyblockRepo"))?;

	Ok(())
}

#[pyfunction(name = "delete_repo")]
pub fn delete_repo_files() -> PyResult<()> {
	let _ = remove_file("SkyblockRepo-main.zip").or_else(|err| {
		// stifle file not found error because you can already remove the zip in the download function
		if err.kind() == io::ErrorKind::NotFound {
			Ok(())
		} else {
			Err(err)
		}
	})?;
	remove_dir_all("SkyblockRepo")?;
	Ok(())
}
