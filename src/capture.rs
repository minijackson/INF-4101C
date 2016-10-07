extern crate rscam;
extern crate image;

use self::rscam::{Camera, Config};
use self::image::{ImageBuffer, Rgba, Rgb, ConvertBuffer};

use std::sync::mpsc::Sender;
use std::thread;

pub fn stream(sender : Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>) {
    thread::spawn(move || {

        let mut cam = Camera::new("/dev/video0").unwrap();

        cam.start(&Config {
            interval: (1, 30),
            resolution: (1280, 720),
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