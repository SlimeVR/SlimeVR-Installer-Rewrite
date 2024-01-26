use std::str::FromStr;

use serde::{de::Error as DeError, Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Platform {
	arch: target_lexicon::Architecture,
	os: target_lexicon::OperatingSystem,
}

impl Platform {
	pub const CURRENT: Self = Self {
		arch: target_lexicon::Architecture::host(),
		os: target_lexicon::OperatingSystem::host(),
	};
}

impl ToString for Platform {
	fn to_string(&self) -> String {
		format!("{}-{}", self.arch, self.os)
	}
}

impl FromStr for Platform {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let v: Vec<&str> = s.split("-").collect();
		if v.len() != 2 {
			return Err(());
		}

		let arch = v[0].parse()?;
		let os = v[1].parse()?;
		Ok(Self { arch, os })
	}
}

impl Serialize for Platform {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for Platform {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = std::string::String::deserialize(deserializer)?;
		string.parse().map_err(|_| {
			D::Error::custom(format!("Unrecognized target triple {}", string))
		})
	}
}
