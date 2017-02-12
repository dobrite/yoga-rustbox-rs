use rand::Rng;

use rand; // not in final version
use rustbox;
use yoga;
use yoga_wrapper;

use yoga::Renders;

use Backend;
use Builder;

pub struct Renderer<'rbox> {
    pub rustbox: &'rbox rustbox::RustBox,
    pub colors: Vec<rustbox::Color>,
}

impl<'rbox> Renderer<'rbox> {
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

    fn walk(&mut self, node: &yoga::Renderable<rustbox::Color>) {
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

        for i in 0..node.get_child_count() {
            let child = node.get_child(i).unwrap();
            self.render(child);
        }
    }
}

impl<'rbox> yoga::Renders for Renderer<'rbox> {
    type Color = rustbox::Color;
    type Builder = Builder;

    fn render(&mut self, node: &yoga::Renderable<Self::Color>) {
        self.walk(node);
        self.rustbox.present();
    }
}
