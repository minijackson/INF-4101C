extern crate piston_window;
extern crate image;

extern crate rustc_serialize;
extern crate docopt;

use std::sync::mpsc::channel;
use std::fs::File;
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
INF-4101C Optimization course.

Usage:
  inf_4101c display [--median-kernel-size=<size>]
  inf_4101c save <prefix> [--median-kernel-size=<size> --count=<n>]
  inf_4101c (-h | --help)
  inf_4101c --version

Options:
  -h --help                    Show this screen.
  --version                    Show version.
  --median-kernel-size=<size>  Set the size of the media filter [default: 5].
  --count=<n>                  Set the number of files to save [default: 10].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_display: bool,
    cmd_save: bool,
    arg_prefix: Option<String>,
    flag_median_kernel_size: usize,
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

        capture::stream(sender);

        while let Some(e) = window.next() {
            if let Ok(frame) = receiver.try_recv() {
                let frame = process_frame(frame.convert(), args.flag_median_kernel_size);
                displayed_texture = display::build_texture(frame.convert(), displayed_texture, &mut window);
            }

            display::show_texture(&displayed_texture, &mut window, e);
        }
    } else if args.cmd_save {
        let prefix = args.arg_prefix.unwrap();
        println!("Saving with prefix: {}", prefix);

        let (sender, receiver) = channel();

        capture::stream(sender);

        for i in 0..args.flag_count {

            let path = format!("{}_{}.png", prefix, i);

            loop {
                match receiver.try_recv() {
                    Ok(frame) => {
                        let frame = process_frame(frame.convert(), args.flag_median_kernel_size);
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

fn process_frame(frame : ImageBuffer<Luma<u8>, Vec<u8>>, median_kernel_size : usize)
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
    //let frame = processing::sobel_optimized(frame.convert());
    //let frame = processing::median_filter(frame.convert());
    //let frame = processing::median_filter_hist(frame.convert(), args.flag_median_kernel_size);
    processing::median_filter_hist_optimized(frame.convert(), median_kernel_size)
}
