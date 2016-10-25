extern crate rscam;
extern crate image;

use self::rscam::{Camera, Config};
use self::image::{
    ImageBuffer,
    Rgba,
    Rgb,
    Luma,
    ConvertBuffer,
    GenericImage
};

use std::sync::mpsc::Sender;
use std::thread;
use std::time;
use std::path::Path;

pub fn stream(sender : Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>, device : String) {
    thread::spawn(move || {

        let mut cam = Camera::new(device.as_str()).unwrap();

        cam.start(&Config {
            interval: (1, 30),
            resolution: (640, 480),
            format: b"RGB3",
            ..Default::default()
        }).unwrap();

        while let Some(frame) = cam.capture().ok() {

            let frame : image::ImageBuffer<Rgb<u8>, _>
                = image::ImageBuffer::from_raw(frame.resolution.0,
                                               frame.resolution.1,
                                               frame).unwrap();

            if let Err(_) = sender.send(frame.convert()) {
                break;
            }
        }
    });
}

pub fn fake_stream(sender : Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>, image : String) {
    thread::spawn(move || {
        let frame = image::open(&Path::new(image.as_str())).unwrap();

        let frame : ImageBuffer<Rgb<u8>, Vec<u8>>
            = ImageBuffer::from_fn(frame.width(), frame.height(), |x, y| frame.get_pixel(x, y)).convert();

        loop {
            if let Err(_) = sender.send(frame.convert()) {
                break;
            }

            thread::sleep(time::Duration::from_millis(5));
        }
    });
}

pub fn capture(device : String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut cam = Camera::new(device.as_str()).unwrap();

    cam.start(&Config {
        interval: (1, 30),
        resolution: (640, 480),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();

    let frame = cam.capture().unwrap();
    let frame : ImageBuffer<Rgb<u8>, _>
        = ImageBuffer::from_raw(frame.resolution.0,
                                frame.resolution.1,
                                frame).unwrap();
    return frame.convert();
}

pub fn fake_capture(image : &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let frame = image::open(&Path::new(image)).unwrap();

    ImageBuffer::from_fn(frame.width(), frame.height(), |x, y| frame.get_pixel(x, y)).convert()
}
