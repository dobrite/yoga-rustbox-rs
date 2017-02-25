use rustbox;
use yoga;
use yoga_wrapper;

use yoga::Renders;

use Backend;
use Builder;

pub struct Renderer<'rbox> {
    rustbox: &'rbox rustbox::RustBox,
}

impl<'rbox> Renderer<'rbox> {
    pub fn new(rustbox: &rustbox::RustBox) -> Renderer {
        Renderer { rustbox: rustbox }
    }

    fn walk(&mut self, node: &yoga::Renderable<rustbox::Color>) {
        let width = node.get_layout_width() as usize;
        let height = node.get_layout_height() as usize;
        let top = node.get_layout_top() as usize;
        let left = node.get_layout_left() as usize;

        let color = node.get_color();
        let background_color = match *node.get_background_color() {
            Some(yoga::style::BackgroundColor::Transparent) => rustbox::Color::Black,
            Some(yoga::style::BackgroundColor::Color(c)) => c,
            None => rustbox::Color::Black,
        };

        for y in top..(top + height) {
            self.rustbox.print(left,
                               y,
                               rustbox::RB_BOLD,
                               rustbox::Color::White, // TODO use color
                               background_color,
                               &format!("{:1$}", "", width));
        }

        for i in 0..node.get_child_count() {
            let child = node.get_child(i).unwrap();
            self.render(child);
        }
    }
}

impl<'rbox, 'meas> yoga::Renders<'meas> for Renderer<'rbox> {
    type Color = rustbox::Color;
    type Builder = Builder;

    fn render(&mut self, node: &yoga::Renderable<Self::Color>) {
        self.walk(node);
        self.rustbox.present();
    }
}
