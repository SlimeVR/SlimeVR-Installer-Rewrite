use std::fmt::Display;

use crate::platform::Platform;

use super::version::VersionResolvable;

#[derive(Debug, Clone)]
pub enum IncompatibilityReason {
	PlatformNotSupported(Platform),
}

#[derive(Debug, Clone)]
pub struct IncompatibleComponent {
	display_name: String,
	reason: IncompatibilityReason,
	version: VersionResolvable,
	platforms: Vec<Platform>,
}

impl IncompatibleComponent {
	pub(super) fn new(
		display_name: String,
		reason: IncompatibilityReason,
		version: VersionResolvable,
		platforms: Vec<Platform>,
	) -> Self {
		Self {
			display_name,
			reason,
			version,
			platforms,
		}
	}

	pub async fn fetch(&self) -> reqwest::Result<()> {
		self.version.fetch().await
	}

	pub fn reason(&self) -> &IncompatibilityReason {
		&self.reason
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

impl Display for IncompatibleComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} v{} because {:?}",
			self.display_name, self.version, self.reason
		)
	}
}
