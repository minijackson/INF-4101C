extern crate piston_window;
extern crate image;
extern crate gfx_device_gl;

use self::piston_window::{
    PistonWindow,
    clear,
    Texture,
    TextureSettings,
    GenericEvent
};

use self::gfx_device_gl::Resources;

use self::image::{
    ImageBuffer,
    Rgba
};

pub fn build_texture(frame  : ImageBuffer<Rgba<u8>, Vec<u8>>,
                     tex    : Option<Texture<Resources>>,
                     window : &mut PistonWindow)
        -> Option<Texture<Resources>> {

    if let Some(mut t) = tex {
        t.update(&mut window.encoder, &frame).unwrap();
        return Some(t);
    } else {
        return Texture::from_image(&mut window.factory, &frame, &TextureSettings::new()).ok();
    }
}

pub fn show_texture<E>(texture : &Option<Texture<Resources>>,
                       window  : &mut PistonWindow,
                       event   : E) 
        where E : GenericEvent {
    window.draw_2d(&event, |context, graphics| {
        clear([1.0; 4], graphics);
        if let Some(ref t) = *texture {
            piston_window::image(t, context.transform, graphics);
        }
    });
}
