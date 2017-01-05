use rustbox;
use yoga;
use yoga_wrapper;

use yoga::Backend as B;
use yoga::Renders as R;

use measurer;
use renderer;

pub struct Backend<'rbox> {
    renderer: renderer::Renderer<'rbox>,
    measurer: measurer::Measurer,
}

impl<'rbox> Backend<'rbox> {
    pub fn new(rustbox: &rustbox::RustBox) -> Backend {
        Backend {
            renderer: renderer::Renderer::new(rustbox),
            measurer: measurer::Measurer {},
        }
    }

    pub fn draw(&self, node: &yoga_wrapper::Node) {
        self.render(&self.renderer, node);
        self.renderer.rustbox.present();
    }
}

impl<'rbox, 'meas> yoga::Backend<'meas> for Backend<'rbox> {
    type Color = i32;
    type Renderer = renderer::Renderer<'rbox>;
    type Measurer = measurer::Measurer;

    fn render(&self, renderer: &renderer::Renderer, node: &yoga_wrapper::Node) {
        renderer.render(node)
    }

    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas> {
        yoga_wrapper::Context::new(text, &self.measurer)
    }
}
