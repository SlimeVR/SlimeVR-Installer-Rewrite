use std::str::FromStr;

use formatx::formatx;
use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::component::MaybeCrossPlatform;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Procedure {
	Download(Download),
	Unzip,
	Run(Run),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Download(MaybeCrossPlatform<String>);

impl Download {
	pub fn get_url(&self, v: Option<semver::Version>) -> Option<reqwest::Url> {
		let Some(v) = v else {
			return None;
		};
		self.0.get().map(|u| {
			reqwest::Url::from_str(&formatx!(u, version = v).unwrap()).unwrap()
		})
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Run(MaybeCrossPlatform<Vec<String>>);

impl Run {
	pub fn get_command(
		&self,
		v: Option<semver::Version>,
	) -> Option<std::process::Command> {
		let Some(v) = v else {
			return None;
		};
		let Some(c) = self.0.get() else {
			return None;
		};
		c.first().expect("Minimum command array length is 1");

		let mut cmd = std::process::Command::new(&c[0]);
		cmd.args(&c[1..]).envs([("VERSION", v.to_string())]);
		Some(cmd)
	}
}
