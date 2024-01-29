use std::{collections::HashMap, fmt::Display, fs::File, io, path::Path};

use futures::future::join_all;
use log::error;
use serde::{Deserialize, Serialize};

use crate::{
	parsing::Procedure,
	platform::Platform,
	util::{Selectable, SelectableHashMap},
};

use super::{
	incompatible::{IncompatibilityReason, IncompatibleComponent},
	version::VersionResolvable,
	MaybeCrossPlatform,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableComponentsFile {
	components: HashMap<String, Component>,
}

impl AvailableComponentsFile {
	pub fn load<P: AsRef<Path>>(path: P) -> io::Result<AvailableComponentsFile> {
		match serde_yaml::from_reader(File::open(path)?) {
			Ok(components) => Ok(components),
			Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
	display_name: String,
	version: VersionResolvable,
	platforms: Vec<Platform>,
	#[serde(with = "serde_yaml::with::singleton_map_recursive")]
	procedure: Vec<Procedure>,
	dependencies: Option<MaybeCrossPlatform<Vec<String>>>,
}

impl Component {
	pub async fn fetch(&self) -> reqwest::Result<()> {
		self.version.fetch().await
	}

	pub fn incompatible_because(
		&self,
		reason: IncompatibilityReason,
	) -> IncompatibleComponent {
		IncompatibleComponent::new(
			self.display_name.clone(),
			reason,
			self.version.clone(),
			self.platforms.clone(),
		)
	}

	pub fn version(&self) -> &VersionResolvable {
		&self.version
	}

	pub fn display_name(&self) -> &str {
		&self.display_name
	}

	pub fn platforms(&self) -> &Vec<Platform> {
		&self.platforms
	}
}

impl Selectable<String> for Component {
	fn dependencies(&self) -> Option<&[String]> {
		match &self.dependencies {
			Some(d) => d.get().map(|v| v.as_slice()),
			None => None,
		}
	}
}

impl Display for Component {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.display_name, self.version)
	}
}

pub struct Components {
	compatible: HashMap<String, Component>,
	incompatible: HashMap<String, IncompatibleComponent>,
}
impl Components {
	pub async fn fetch(&self) {
		join_all(self.compatible.iter().map(|(name, c)| async move {
			if let Err(e) = c.fetch().await {
				error!("\"{name}\" couldn't fetch version: {e}")
			}
		}))
		.await;

		join_all(self.incompatible.iter().map(|(name, c)| async move {
			if let Err(e) = c.fetch().await {
				error!("\"{name}\" couldn't fetch version {e}")
			}
		}))
		.await;
	}

	pub fn compatible(&self) -> &HashMap<String, Component> {
		&self.compatible
	}

	pub fn incompatible(&self) -> &HashMap<String, IncompatibleComponent> {
		&self.incompatible
	}

	/// Converts the `Components` into a `SelectableHashMap` of all compatible `Component`s.
	pub fn into_selectable_hashmap(self) -> SelectableHashMap<String, Component> {
		SelectableHashMap::new(self.compatible)
	}
}

impl From<AvailableComponentsFile> for Components {
	fn from(available: AvailableComponentsFile) -> Self {
		let mut compatible = HashMap::new();
		let mut incompatible = HashMap::new();

		for (name, component) in available.components {
			if component.platforms.contains(&Platform::CURRENT) {
				compatible.insert(name, component);
			} else {
				incompatible.insert(
					name,
					component.incompatible_because(
						IncompatibilityReason::PlatformNotSupported(Platform::CURRENT),
					),
				);
			}
		}

		Components {
			compatible,
			incompatible,
		}
	}
}
