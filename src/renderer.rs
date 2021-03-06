use Builder;
use rustbox;
use yoga;

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

        let color = match *node.get_color() {
            Some(c) => c,
            None => rustbox::Color::White,
        };

        let background_color = match *node.get_background_color() {
            Some(yoga::style::BackgroundColor::Transparent) => rustbox::Color::Black,
            Some(yoga::style::BackgroundColor::Color(c)) => c,
            None => rustbox::Color::Black,
        };

        if let Some(text) = node.get_text() {
            self.rustbox
                .print(left, top, rustbox::RB_BOLD, color, background_color, text);
        } else {
            for y in top..(top + height) {
                self.rustbox
                    .print(
                        left,
                        y,
                        rustbox::RB_BOLD,
                        color,
                        background_color,
                        &format!("{:1$}", "", width),
                    );
            }
        }

        for i in 0..node.get_child_count() {
            let child = node.get_child(i).unwrap();
            self.walk(child);
        }
    }
}

impl<'rbox, 'meas> yoga::Renders<'meas> for Renderer<'rbox> {
    type Color = rustbox::Color;
    type Input = Option<bool>;
    type Output = Option<bool>;
    type Builder = Builder;

    fn render(
        &mut self,
        node: &yoga::Renderable<Self::Color>,
        _input: Self::Input,
    ) -> Self::Output {
        self.walk(node);
        self.rustbox.present();
        None
    }
}
