#![feature(test)]
extern crate test;

extern crate piston_window;
extern crate image;

extern crate rustc_serialize;
extern crate docopt;

use std::sync::mpsc::channel;
use std::path::Path;

mod capture;
mod display;
mod processing;

use docopt::Docopt;

use self::piston_window::{
    PistonWindow,
    WindowSettings,
    Texture
};

use self::image::{
    ConvertBuffer,
    ImageBuffer,
    Luma
};

const USAGE: &'static str = "
INF-4101C Optimization project.

Usage:
  inf_4101c display [--fake=<image>] [--device=<video-device>] [--median-kernel-size=<size>] [--threshold=<n>]
  inf_4101c save <prefix> [--fake=<image>] [--device=<video-device>] [--median-kernel-size=<size>] [--count=<n>] [--threshold=<n>]
  inf_4101c (-h | --help)
  inf_4101c --version

Options:
  -h --help                    Show this screen.
  --version                    Show version.
  --fake=<image>               Fake the video stream with an image.
  --device=<video-device>      Set the video device to use for the webcam [default: /dev/video0].
  --median-kernel-size=<size>  Set the size of the media filter [default: 5].
  --threshold=<n>              Set the final threshold value [default: 127].
  --count=<n>                  Set the number of files to save [default: 10].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_display: bool,
    cmd_save: bool,
    arg_prefix: Option<String>,
    flag_fake: Option<String>,
    flag_device: String,
    flag_median_kernel_size: usize,
    flag_threshold: u8,
    flag_count: usize,
    flag_version: bool,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {

    let args : Args = Docopt::new(USAGE)
                              .and_then(|d| d.decode())
                              .unwrap_or_else(|e| e.exit());

    let mut window : PistonWindow;

    if args.flag_version {
        println!("Version: {}", VERSION);
        return;
    }

    if args.cmd_display {

        window = WindowSettings::new("Piston: image", [300, 300])
                  .exit_on_esc(true)
                  .build()
                  .unwrap();

        let mut displayed_texture : Option<Texture<_>> = None;

        let (sender, receiver) = channel();

        if let Some(image) = args.flag_fake {
            capture::fake_stream(sender, image);
        } else {
            capture::stream(sender, args.flag_device);
        }

        while let Some(e) = window.next() {
            if let Ok(frame) = receiver.try_recv() {
                let frame = process_frame(frame.convert(), args.flag_median_kernel_size, args.flag_threshold);
                displayed_texture = display::build_texture(frame.convert(), displayed_texture, &mut window);
            }

            display::show_texture(&displayed_texture, &mut window, e);
        }
    } else if args.cmd_save {
        let prefix = args.arg_prefix.unwrap();
        println!("Saving with prefix: {}", prefix);

        let (sender, receiver) = channel();

        if let Some(image) = args.flag_fake {
            capture::fake_stream(sender, image);
        } else {
            capture::stream(sender, args.flag_device);
        }

        for i in 0..args.flag_count {

            let path = format!("{}_{}.png", prefix, i);

            loop {
                match receiver.try_recv() {
                    Ok(frame) => {
                        println!("Saving frame {}", path);
                        let frame = process_frame(frame.convert(), args.flag_median_kernel_size, args.flag_threshold);
                        let _     = frame.save(&Path::new(&path)).unwrap();
                        break;
                    },
                    _ => {}
                }
            }
        }
    } else {
        unreachable!("Neither display, neither save, I don't know what to do…");
    }
}

fn process_frame(frame : ImageBuffer<Luma<u8>, Vec<u8>>, median_kernel_size : usize, threshold : u8)
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let frame = processing::median_filter_hist_optimized(frame.convert(), median_kernel_size);
    processing::sobel_and_threshold(frame.convert(), threshold)
}

#[cfg(test)]
mod tests {
    use ::test::Bencher;
    use image::{
        ConvertBuffer,
        ImageBuffer,
        Luma
    };
    use ::capture;

    #[bench]
    fn process_frame_3(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 3, 127)
        });
    }

    #[bench]
    fn process_frame_5(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 5, 127)
        });
    }

    #[bench]
    fn process_frame_9(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 9, 127)
        });
    }

    #[bench]
    fn process_frame_15(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 15, 127)
        });
    }

    #[bench]
    fn process_frame_21(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 21, 127)
        });
    }

    #[bench]
    fn process_frame_31(b : &mut Bencher) {
        let frame : ImageBuffer<Luma<u8>, Vec<u8>> = capture::capture(String::from("/dev/video0")).convert();

        b.iter(|| {
            let frame = frame.convert();
            super::process_frame(frame, 31, 127)
        });
    }

}
