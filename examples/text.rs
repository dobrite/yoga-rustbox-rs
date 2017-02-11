extern crate rustbox;
extern crate yoga;
extern crate yoga_rustbox;
extern crate yoga_wrapper;

use yoga::{Backend, Renders, Renderable};

use std::error::Error;
use std::default::Default;

use rustbox::RustBox;
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let text = yoga::Text::new("Yo!!!");

    let mut root = yoga::View::new();
    root.set_width(50.0);
    root.set_height(12.0);
    root.set_flex_direction(yoga_wrapper::FlexDirection::Row);
    root.set_padding(yoga_wrapper::Edge::All, 2.0);

    root.insert_child(&text, 0);

    root.calculate_layout();

    yoga_rustbox::Backend::new(&rustbox).get_renderer().render(&root);

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
