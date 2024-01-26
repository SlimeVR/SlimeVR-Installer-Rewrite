use std::{collections::HashMap, fs::File, io, path::Path};

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledComponentsFile {
	components: HashMap<String, InstalledComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledComponent {
	version: Version,
}

impl InstalledComponentsFile {
	pub fn load<P: AsRef<Path>>(path: P) -> io::Result<InstalledComponentsFile> {
		match path.as_ref().exists() {
			true => {
				let file = File::open(path)?;
				match serde_yaml::from_reader(file) {
					Ok(components) => Ok(components),
					Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
				}
			}
			false => Ok(InstalledComponentsFile {
				components: HashMap::new(),
			}),
		}
	}

	pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
		let file = File::create(path)?;
		match serde_yaml::to_writer(file, self) {
			Ok(_) => Ok(()),
			Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
		}
	}
}
