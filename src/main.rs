extern crate piston_window;

use std::sync::mpsc::channel;

mod capture;
mod display;

use self::piston_window::{
    PistonWindow,
    WindowSettings,
    Texture
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
            displayed_texture = display::build_texture(frame, displayed_texture, &mut window);
        }

        display::show_texture(&displayed_texture, &mut window, e);
    }
}
