extern crate piston_window;
extern crate image;

use std::sync::mpsc::channel;

mod capture;
mod display;
mod processing;

use self::piston_window::{
    PistonWindow,
    WindowSettings,
    Texture
};

use self::image::{
    ConvertBuffer
};

fn main() {

    let mut window : PistonWindow =
        WindowSettings::new("Piston: image", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut displayed_texture : Option<Texture<_>> = None;

    let (sender, receiver) = channel();

    capture::stream(sender);

    while let Some(e) = window.next() {
        if let Ok(frame) = receiver.try_recv() {
            //let frame = processing::sobel_optimized(frame.convert());
            //let frame = processing::median_filter(frame.convert());
            //let frame = processing::median_filter_hist(frame.convert(), 9);
            let frame = processing::median_filter_hist_optimized(frame.convert(), 9);
            displayed_texture = display::build_texture(frame.convert(), displayed_texture, &mut window);
        }

        display::show_texture(&displayed_texture, &mut window, e);
    }
}
