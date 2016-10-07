use std::sync::mpsc::channel;

mod capture;
mod display;

fn main() {
    let (sender, receiver) = channel();

    capture::stream(sender);
    display::listener(receiver);
}
