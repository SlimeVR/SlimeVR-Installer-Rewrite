use fltk::{
	enums::Color,
	image::{AnimGifImage, AnimGifImageFlags},
	prelude::*,
};
use rust_embed::RustEmbed;

pub const OUTER_BACKGROUND: Color = Color::from_hex(0x00101C);
pub const ACTUAL_INNER_BACKGROUND: Color = Color::from_hex(0x112D43);
pub const INNER_BACKGROUND: Color = Color::from_hex(0x1A3D59);
pub const TEXT: Color = Color::from_hex(0x78A4C6);
pub const ACCENT_COLOR: Color = Color::from_hex(0x9D5CD4);

pub const WIDTH: i32 = 400;
pub const HEIGHT: i32 = 300;

pub fn create_gif<W: WidgetExt>(
	data: &[u8],
	w: &mut W,
) -> Result<AnimGifImage, FltkError> {
	let mut img = AnimGifImage::from_data(
		data,
		w,
		AnimGifImageFlags::DONT_RESIZE_CANVAS | AnimGifImageFlags::DONT_SET_AS_IMAGE,
	);
	if let Ok(img) = &mut img {
		img.scale(w.w(), w.h(), true, true);
	};
	img
}

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;
