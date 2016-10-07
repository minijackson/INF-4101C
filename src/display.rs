extern crate piston_window;
extern crate image;

use self::piston_window::{PistonWindow, WindowSettings, clear, Texture, TextureSettings};
use self::image::{ImageBuffer, Rgba, ConvertBuffer};

use std::sync::mpsc::Receiver;

pub fn listener(receiver : Receiver<ImageBuffer<Rgba<u8>, Vec<u8>>>) {
    let mut window : PistonWindow =
        WindowSettings::new("Piston: image", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex : Option<Texture<_>> = None;

    while let Some(e) = window.next() {
        if let Ok(frame) = receiver.try_recv() {

            if let Some(mut t) = tex {
                t.update(&mut window.encoder, &frame).unwrap();
                tex = Some(t);
            } else {
                tex = Texture::from_image(&mut window.factory, &frame, &TextureSettings::new()).ok();
            }
        }

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            if let Some(ref t) = tex {
                piston_window::image(t, c.transform, g);
            }
        });
    }
}
