use rand::Rng;

use rand; // not in final version
use rustbox;
use yoga;
use yoga_wrapper;

pub struct Renderer<'r> {
    pub rustbox: &'r rustbox::RustBox,
    pub colors: Vec<rustbox::Color>,
}

impl<'r> Renderer<'r> {
    pub fn new(rustbox: &rustbox::RustBox) -> Renderer {
        Renderer {
            rustbox: rustbox,
            // not in final version
            colors: vec![rustbox::Color::Black,
                         rustbox::Color::Red,
                         rustbox::Color::Green,
                         rustbox::Color::Yellow,
                         rustbox::Color::Blue,
                         rustbox::Color::Magenta,
                         rustbox::Color::Cyan,
                         rustbox::Color::White],
        }
    }
}

impl<'r> yoga::Renders for Renderer<'r> {
    fn render(&self, node: &yoga_wrapper::Node) {
        // maybe take HL Node?
        let ct = node.get_child_count();

        let width = node.get_layout_width() as usize;
        let height = node.get_layout_height() as usize;
        let top = node.get_layout_top() as usize;
        let left = node.get_layout_left() as usize;

        let color = *rand::thread_rng().choose(&self.colors).unwrap();

        for y in top..(top + height) {
            self.rustbox.print(left,
                               y,
                               rustbox::RB_BOLD,
                               rustbox::Color::White,
                               color,
                               &format!("{:1$}", "", width));
        }

        for i in 0..ct {
            let child = node.get_child(i);
            self.render(&child);
        }
    }
}
