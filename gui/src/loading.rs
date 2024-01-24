use std::{cell::RefCell, rc::Rc};

use fltk::{
    app::{self},
    frame::{self, Frame},
    group,
    misc::{self, Progress},
    prelude::*,
    widget_extends,
};

use crate::util;

const ANIMATION_DELAY: f64 = 2.;

fn animate_status_text(
    mut status_label: Frame,
    text: Rc<RefCell<String>>,
    i: Rc<RefCell<usize>>,
    handle: app::TimeoutHandle,
) {
    const DOTS_ARRAY: [&str; 4] = ["", ".", "..", "..."];
    if !status_label.visible_r() {
        app::repeat_timeout3(ANIMATION_DELAY, handle);
        return;
    }
    {
        let mut i = i.borrow_mut();
        status_label.set_label(&format!("{}{}", text.borrow(), DOTS_ARRAY[*i]));
        *i += 1;
        if *i == 4 {
            *i = 0
        }
    }
    status_label.parent().unwrap().redraw();
    app::repeat_timeout3(ANIMATION_DELAY, handle);
}

pub struct LoadingScreen {
    inner: group::Group,
    progress: Progress,
    status: Frame,
    text: Rc<RefCell<String>>,
}

impl LoadingScreen {
    pub fn new() -> Self {
        let g = group::Group::default_fill();
        let mut progress = misc::Progress::default()
            .with_size(300, 8)
            .with_pos(0, util::HEIGHT - 10 * 2)
            .center_x(&g);
        progress.set_minimum(0.);
        progress.set_maximum(100.);
        progress.set_value(50.);
        let mut status = frame::Frame::default()
            .with_size(5, 20)
            .above_of(&progress, 5)
            .center_x(&progress);
        status.set_label_size(16);
        status.set_label("");
        g.end();

        let text = Rc::new(RefCell::new(String::from("Installing SlimeVR Server")));
        let animation_index = Rc::new(RefCell::new(0));

        progress.set_selection_color(util::ACCENT_COLOR);
        {
            let status = status.clone();
            let text = text.clone();
            let i = animation_index.clone();
            app::add_timeout3(ANIMATION_DELAY, move |handle| {
                let status = status.clone();
                let text = text.clone();
                let i = i.clone();
                animate_status_text(status, text, i, handle)
            });
        }

        Self {
            inner: g,
            progress,
            status,
            text,
        }
    }

    pub fn progress_value(&self) -> f64 {
        self.progress.value()
    }

    pub fn set_progress_value(&mut self, v: f64) {
        self.progress.set_value(v);
    }

    pub fn status(&self) -> String {
        self.text.borrow().to_string()
    }

    pub fn set_status(&mut self, v: String) {
        self.status.set_label(&v);
        *self.text.borrow_mut() = v;
    }
}

widget_extends!(LoadingScreen, group::Group, inner);
