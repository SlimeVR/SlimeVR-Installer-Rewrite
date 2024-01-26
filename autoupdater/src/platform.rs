use std::str::FromStr;

use lazy_static::lazy_static;
use serde::{de::Error as DeError, Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Platform {
	arch: platforms::Arch,
	os: platforms::OS,
}

const TARGET: &str = env!("TARGET");

impl Platform {
	pub fn current() -> Self {
		lazy_static! {
			static ref CURRENT: &'static platforms::Platform =
				platforms::Platform::find(TARGET).unwrap();
		}

		Self {
			arch: CURRENT.target_arch,
			os: CURRENT.target_os,
		}
	}

	fn as_str(self) -> String {
		format!("{}-{}", self.arch, self.os)
	}
}

impl FromStr for Platform {
	type Err = platforms::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let v: Vec<&str> = s.split("-").collect();
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
		serializer.serialize_str(&self.as_str())
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
