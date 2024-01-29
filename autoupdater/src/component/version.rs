use std::{cell::OnceCell, fmt::Display};

use color_eyre::Result;
use log::warn;
use semver::Version;
use serde::{de, Deserialize, Serialize};

use crate::util;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VersionResolvable {
	#[serde(alias = "github")]
	Github(GithubRepo),
	#[serde(alias = "java")]
	Java(JavaAdoptium),

	// Can't make version be untagged because of https://github.com/serde-rs/serde/issues/1183
	// #[serde(untagged)]
	#[serde(alias = "semver")]
	Semver(Version),
}

impl VersionResolvable {
	pub async fn fetch(&self) -> reqwest::Result<()> {
		match self {
			Self::Github(r) => {
				r.fetch_version().await?;
			}
			Self::Java(j) => {
				j.fetch_version().await?;
			}
			Self::Semver(_) => {}
		};
		Ok(())
	}

	pub fn latest(&self) -> Option<Version> {
		match self {
			Self::Github(r) => r.latest(),
			Self::Java(j) => Some(j.latest()),
			Self::Semver(v) => Some(v.clone()),
		}
	}
}

// TODO: Don't implement display while using .latest(),
//		maybe we don't want to get latest always.
impl Display for VersionResolvable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Github(r) => {
				if let Some(v) = r.latest() {
					write!(f, "v{v}")
				} else {
					write!(f, "unknown version")
				}
			}
			Self::Java(j) => write!(f, "v{}", j.latest()),
			Self::Semver(v) => write!(f, "v{v}"),
		}
	}
}

fn deserialize_version_v<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
	D: de::Deserializer<'de>,
{
	let buf = String::deserialize(deserializer)?;

	if !buf.starts_with('v') {
		return Err(de::Error::custom("Version doesn't start with v"));
	}

	Version::parse(&buf[1..]).map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRelease {
	#[serde(deserialize_with = "deserialize_version_v")]
	pub tag_name: Version,
	pub draft: bool,
	pub prerelease: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GithubRepo(String, #[serde(skip)] OnceCell<Vec<GithubRelease>>);

impl GithubRepo {
	fn get_releases(&self) -> Option<&Vec<GithubRelease>> {
		let releases = self.1.get();
		if releases.is_none() {
			warn!("Tried to get a non-yet fetched version in {}", self.0)
		}
		releases
	}

	async fn fetch_version(&self) -> reqwest::Result<&Vec<GithubRelease>> {
		if let Some(releases) = self.1.get() {
			return Ok(releases);
		}
		let client = util::CLIENT.clone();
		let releases = client
			.get(format!("https://api.github.com/repos/{}/releases", self.0))
			.send()
			.await?
			.json()
			.await?;
		Ok(self.1.get_or_init(move || releases))
	}

	fn latest(&self) -> Option<Version> {
		let Some(releases) = self.get_releases() else {
			return None;
		};

		let Some(r) = releases.iter().find(|r| !r.draft && !r.prerelease) else {
			warn!("Couldn't find any version in {}", self.0);
			return None;
		};

		Some(r.tag_name.clone())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdoptiumRelease {
	version_data: AdoptiumVersionData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdoptiumVersionData {
	semver: Version,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JavaAdoptium(u8, #[serde(skip)] OnceCell<AdoptiumRelease>);

impl JavaAdoptium {
	fn get_arch() -> &'static str {
		match std::env::consts::ARCH {
			"x86_64" => "x64",
			arch => arch,
		}
	}

	fn get_os() -> &'static str {
		match std::env::consts::OS {
			"macos" => "mac",
			os => os,
		}
	}

	fn get_releases(&self) -> Option<&AdoptiumRelease> {
		let releases = self.1.get();
		if releases.is_none() {
			warn!("Tried to get a non-yet fetched version in {}", self.0)
		}
		releases
	}

	fn generate_version_link(&self) -> reqwest::Url {
		reqwest::Url::parse_with_params(
			&format!(
				"https://api.adoptium.net/v3/assets/feature_releases/{}/ga",
				self.0
			),
			&[
				("architecture", Self::get_arch()),
				("os", Self::get_os()),
				("heap_size", "normal"),
				("image_type", "jre"),
				("page", "0"),
				("page_size", "1"),
				("project", "jdk"),
				("sort_method", "DEFAULT"),
				("sort_order", "DESC"),
				("vendor", "eclipse"),
			],
		)
		.unwrap()
	}

	async fn fetch_version(&self) -> reqwest::Result<&AdoptiumRelease> {
		if let Some(releases) = self.1.get() {
			return Ok(releases);
		}
		let client = util::CLIENT.clone();
		let [release]: [AdoptiumRelease; 1] = client
			.get(self.generate_version_link())
			.send()
			.await?
			.json()
			.await?;
		Ok(self.1.get_or_init(move || release))
	}

	fn latest(&self) -> Version {
		if let Some(release) = self.get_releases() {
			release.version_data.semver.clone()
		} else {
			Version::new(self.0.into(), 0, 0)
		}
	}
}
