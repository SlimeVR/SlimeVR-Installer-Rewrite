mod selectable_hashmap;

use lazy_static::lazy_static;
pub use selectable_hashmap::Selectable;
pub use selectable_hashmap::SelectableHashMap;

lazy_static! {
	pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
		.user_agent("SlimeVR-Installer")
		.build()
		.unwrap();
}
