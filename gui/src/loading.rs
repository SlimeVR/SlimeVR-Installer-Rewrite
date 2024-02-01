use std::{cell::RefCell, rc::Rc};

use fltk::{
	app::{self},
	button,
	frame::Frame,
	group,
	image::AnimGifImage,
	misc::Progress,
	prelude::*,
	widget_extends,
};
use vek::Lerp;

use crate::util::{self, create_gif, Assets};

pub struct LoadingScreen {
	inner: group::Group,

	pub progress: AnimatedProgress,
	pub status: AnimatedText,
	pub animated_slime: AnimatedSlime,
}

impl LoadingScreen {
	pub fn new() -> Self {
		let g = group::Group::default_fill();
		let mut progress = AnimatedProgress::new()
			.with_size(300, 8)
			.with_pos(0, util::HEIGHT - 10 * 2)
			.center_x(&g);
		progress.set_minimum(0.);
		progress.set_maximum(100.);

		let mut status = AnimatedText::new([
			"".to_string(),
			".".to_string(),
			"..".to_string(),
			"...".to_string(),
		])
		.with_size(5, 20)
		.above_of(&*progress, 5)
		.center_x(&*progress);
		status.set_status("Looking for butter".to_string());

		let animated_slime = AnimatedSlime::new(100, 100)
			.above_of(&*status, 1)
			.center_x(&g);

		let mut button = button::Button::default()
			.with_label("0%")
			.with_size(500, 20);
		{
			let progress = progress.clone();
			let animated_slime = animated_slime.clone();
			button.set_callback(move |_| {
				let mut animated_slime = animated_slime.clone();
				let progress = progress.clone();
				progress.set_progress_value(0.);
				animated_slime.set_status(SlimeState::Happy);
			});
		}
		let mut button_50 = button::Button::default()
			.with_label("50%")
			.with_size(500, 20)
			.below_of(&button, 5);
		{
			let progress = progress.clone();
			let animated_slime = animated_slime.clone();
			button_50.set_callback(move |_| {
				let mut animated_slime = animated_slime.clone();
				let progress = progress.clone();
				progress.set_progress_value(50.);
				animated_slime.set_status(SlimeState::Jumpy);
			});
		}
		let mut button_75 = button::Button::default()
			.with_label("75%")
			.with_size(500, 20)
			.below_of(&button_50, 5);
		{
			let progress = progress.clone();
			let animated_slime = animated_slime.clone();
			button_75.set_callback(move |_| {
				let mut animated_slime = animated_slime.clone();
				let progress = progress.clone();
				progress.set_progress_value(75.);
				animated_slime.set_status(SlimeState::Curious)
			});
		}

		g.end();

		progress.set_progress_value(50.);

		Self {
			inner: g,
			progress,
			status,
			animated_slime,
		}
	}
}
widget_extends!(LoadingScreen, group::Group, inner);

#[derive(Clone, Copy)]
pub enum SlimeState {
	Curious,
	Happy,
	Jumpy,
	Sad,
}

#[derive(Clone)]
pub struct AnimatedSlime {
	inner: Frame,

	status: SlimeState,
	jumping_slime: AnimGifImage,
	curious_slime: AnimGifImage,
	happy_slime: AnimGifImage,
	sad_slime: AnimGifImage,
}

impl AnimatedSlime {
	pub fn new(width: i32, height: i32) -> Self {
		let mut inner = Frame::default().with_size(width, height);
		inner.set_frame(fltk::enums::FrameType::FlatBox);
		let jumping_slime =
			create_gif(&Assets::get("jumping-slime.gif").unwrap().data, &mut inner)
				.unwrap();
		let curious_slime =
			create_gif(&Assets::get("curious-slime.gif").unwrap().data, &mut inner)
				.unwrap();
		let happy_slime =
			create_gif(&Assets::get("happy-slime.gif").unwrap().data, &mut inner)
				.unwrap();
		let sad_slime =
			create_gif(&Assets::get("sad-slime.gif").unwrap().data, &mut inner)
				.unwrap();
		inner.set_image(Some(happy_slime.clone()));

		Self {
			inner,
			status: SlimeState::Happy,
			jumping_slime,
			curious_slime,
			happy_slime,
			sad_slime,
		}
	}

	fn slime_img(&self) -> AnimGifImage {
		match self.status {
			SlimeState::Curious => self.curious_slime.clone(),
			SlimeState::Happy => self.happy_slime.clone(),
			SlimeState::Sad => self.sad_slime.clone(),
			SlimeState::Jumpy => self.jumping_slime.clone(),
		}
	}

	pub fn get_status(&self) -> SlimeState {
		self.status
	}

	pub fn set_status(&mut self, v: SlimeState) {
		self.status = v;
		let img = self.slime_img();
		self.inner.set_image(Some(img))
	}
}
widget_extends!(AnimatedSlime, Frame, inner);

pub struct AnimatedText {
	inner: Frame,

	text: Rc<RefCell<String>>,
	handle: app::TimeoutHandle,
}

impl AnimatedText {
	pub fn new<const N: usize>(states: [String; N]) -> Self {
		let mut inner = Frame::default();
		inner.set_label("");

		let text = Rc::new(RefCell::new(String::from("")));
		let handle = {
			let inner = inner.clone();
			let text = text.clone();
			let states = Rc::new(states);
			let i = Rc::new(RefCell::new(0));
			app::add_timeout3(Self::ANIMATION_DELAY, move |handle| {
				Self::animate_status_text(
					inner.clone(),
					text.clone(),
					i.clone(),
					states.clone(),
					handle,
				)
			})
		};

		Self {
			inner,
			text,
			handle,
		}
	}

	const ANIMATION_DELAY: f64 = 2.;
	fn animate_status_text<const N: usize>(
		mut status_label: Frame,
		text: Rc<RefCell<String>>,
		i: Rc<RefCell<usize>>,
		states: Rc<[String; N]>,
		handle: app::TimeoutHandle,
	) {
		if !status_label.visible_r() {
			app::repeat_timeout3(Self::ANIMATION_DELAY, handle);
			return;
		}
		{
			let mut i = i.borrow_mut();
			status_label.set_label(&format!("{}{}", text.borrow(), states[*i]));
			*i += 1;
			if *i == N {
				*i = 0
			}
		}
		status_label.parent().unwrap().redraw();
		app::repeat_timeout3(Self::ANIMATION_DELAY, handle);
	}

	pub fn status(&self) -> String {
		self.text.borrow().to_string()
	}

	pub fn set_status(&mut self, v: String) {
		self.inner.set_label(&v);
		*self.text.borrow_mut() = v;
	}
}
widget_extends!(AnimatedText, Frame, inner);

#[derive(Clone)]
pub struct AnimatedProgress {
	inner: Progress,

	prev_progress_value: Rc<RefCell<f64>>,
	progress_value: Rc<RefCell<f64>>,
	iteration: Rc<RefCell<f64>>,

	handle: app::TimeoutHandle,
}

impl AnimatedProgress {
	const TOTAL_PROGRESS_TIME: f64 = 0.8;
	const ITERATION_ANIMATION: f64 = 0.005;
	pub fn new() -> Self {
		let mut inner = Progress::default();

		inner.set_selection_color(util::ACCENT_COLOR);

		let progress_value = Rc::new(RefCell::new(0.));
		let prev_progress_value = Rc::new(RefCell::new(0.));
		let iteration = Rc::new(RefCell::new(1.));
		let handle = {
			let inner = inner.clone();
			let progress_value = progress_value.clone();
			let prev_progress_value = prev_progress_value.clone();
			let iteration = iteration.clone();
			app::add_timeout3(Self::ITERATION_ANIMATION, move |handle| {
				Self::animate_progress_bar(
					inner.clone(),
					progress_value.clone(),
					prev_progress_value.clone(),
					iteration.clone(),
					handle,
				)
			})
		};

		Self {
			inner,
			progress_value,
			prev_progress_value,
			iteration,
			handle,
		}
	}

	fn animate_progress_bar(
		mut progress: Progress,
		progress_value: Rc<RefCell<f64>>,
		prev_progress_value: Rc<RefCell<f64>>,
		time: Rc<RefCell<f64>>,
		handle: app::TimeoutHandle,
	) {
		let second_per = Self::TOTAL_PROGRESS_TIME / progress.maximum();
		let time_factor = *time.borrow()
			/ (second_per
				* (*progress_value.borrow() - *prev_progress_value.borrow()).abs());
		if !progress.visible_r() {
			app::repeat_timeout3(2., handle);
			return;
		}
		if time_factor >= 1. {
			app::repeat_timeout3(0.2, handle);
			return;
		}

		*time.borrow_mut() += Self::ITERATION_ANIMATION;
		let time_factor = *time.borrow()
			/ (second_per
				* (*progress_value.borrow() - *prev_progress_value.borrow()).abs());

		progress.set_value(f64::lerp(
			*prev_progress_value.borrow(),
			*progress_value.borrow(),
			time_factor,
		));
		app::repeat_timeout3(Self::ITERATION_ANIMATION, handle)
	}

	pub fn progress_value(&self) -> f64 {
		*self.progress_value.borrow()
	}

	pub fn set_progress_value(&self, v: f64) {
		let second_per = Self::TOTAL_PROGRESS_TIME / self.inner.maximum();
		let old_value = self.progress_value.replace(v);
		let time_factor = *self.iteration.borrow()
			/ (second_per * (old_value - *self.prev_progress_value.borrow()).abs());
		if time_factor >= 1. || old_value > v {
			*self.iteration.borrow_mut() = 0.;
			*self.prev_progress_value.borrow_mut() = old_value;
		} else {
			let from = *self.prev_progress_value.borrow();
			*self.iteration.borrow_mut() =
				(self.inner.value() * second_per) - (from * second_per);
		}
	}

	pub fn reset(&mut self) {
		*self.iteration.borrow_mut() = 1.;
		let min = self.inner.minimum();
		*self.prev_progress_value.borrow_mut() = min;
		*self.progress_value.borrow_mut() = min;
		self.inner.set_value(min);
	}
}
widget_extends!(AnimatedProgress, Progress, inner);
