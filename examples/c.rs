extern crate rustbox;
extern crate yoga;
extern crate yoga_rustbox;
extern crate yoga_wrapper;

use yoga::{Backend, Builds, Renders, Renderable};

use std::error::Error;
use std::default::Default;

use rustbox::RustBox;
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let builder = yoga_rustbox::Builder::new();

    let mut text = builder.text("Yo!");
    text.set_height(3.0);
    text.set_align_self(yoga_wrapper::Align::Center);
    text.set_flex_grow(1.0);
    text.set_background_color(Some(yoga::style::BackgroundColor::Color(rustbox::Color::Cyan)));

    let mut image = builder.view();
    image.set_width(8.0);
    image.set_margin(yoga_wrapper::Edge::End, 2.0);
    image.set_background_color(Some(yoga::style::BackgroundColor::Color(rustbox::Color::Cyan)));

    let mut root = builder.view();
    root.set_width(50.0);
    root.set_height(12.0);
    root.set_flex_direction(yoga_wrapper::FlexDirection::Row);
    root.set_padding(yoga_wrapper::Edge::All, 2.0);
    root.set_background_color(Some(yoga::style::BackgroundColor::Color(rustbox::Color::White)));

    root.insert_child(&image, 0);
    root.insert_child(&text, 1);

    root.calculate_layout();

    yoga_rustbox::Backend::new(&rustbox).render(&root);

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
