use std::{cell::OnceCell, fmt::Display, str::FromStr};

use color_eyre::Result;
use log::warn;
use semver::Version;
use serde::{de, Deserialize, Serialize};

use crate::util;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VersionResolvable {
	Github(GithubRepo),
	Semver(Version),
}

impl VersionResolvable {
	pub async fn fetch(&self) -> reqwest::Result<()> {
		match self {
			Self::Github(r) => {
				r.fetch_version().await?;
			}
			Self::Semver(_) => {}
		};
		Ok(())
	}

	pub fn latest(&self) -> Option<Version> {
		match self {
			Self::Github(r) => r.latest(),
			Self::Semver(v) => Some(v.clone()),
		}
	}
}

impl Display for VersionResolvable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Github(r) => {
				let v = r
					.latest()
					.map(|o| o.to_string())
					.unwrap_or_else(|| "unknown version".to_string());
				write!(f, "v{}", v)
			}
			Self::Semver(v) => write!(f, "v{}", v),
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

#[derive(Clone, Debug)]
pub struct GithubRepo(String, OnceCell<Vec<GithubRelease>>);

impl GithubRepo {
	fn get_releases(&self) -> Option<&Vec<GithubRelease>> {
		let releases = self.1.get();
		if releases.is_none() {
			warn!("Tried to get a non-yet fetched version in {}", self)
		}
		releases
	}

	pub async fn fetch_version(&self) -> reqwest::Result<&Vec<GithubRelease>> {
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

	pub fn latest(&self) -> Option<Version> {
		let Some(releases) = self.get_releases() else {
			return None;
		};

		let Some(r) = releases.iter().find(|r| !r.draft && !r.prerelease) else {
			warn!("Couldn't find any version in {}", self);
			return None;
		};

		Some(r.tag_name.clone())
	}
}

impl Display for GithubRepo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "github:{}", self.0)
	}
}

impl FromStr for GithubRepo {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if !s.starts_with("github:") || s.split('/').take(3).count() != 2 {
			return Err(());
		}
		Ok(Self(s[7..].to_string(), OnceCell::new()))
	}
}

impl Serialize for GithubRepo {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for GithubRepo {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		use serde::de::Error as DeError;
		let string = std::string::String::deserialize(deserializer)?;
		string
			.parse()
			.map_err(|_| D::Error::custom(format!("Invalid Github repo {}", string)))
	}
}
