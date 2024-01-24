#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use fltk::{app, prelude::*, window::Window};
use loading::LoadingScreen;

mod loading;
mod util;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, util::WIDTH, util::HEIGHT, "SlimeVR Installer");
    let mut load_screen = LoadingScreen::new();
    // load_screen.hide();
    wind.end();
    wind.show();

    let (r, g, b) = util::OUTER_BACKGROUND.to_rgb();
    app::background(r, g, b);
    let (r, g, b) = util::INNER_BACKGROUND.to_rgb();
    app::background2(r, g, b);

    let (r, g, b) = util::TEXT.to_rgb();
    app::foreground(r, g, b);

    a.run().unwrap();
}
