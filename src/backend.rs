use rustbox;
use yoga;
use yoga_wrapper;

use yoga::Renders;

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
}

impl<'rbox, 'meas> yoga::Backend<'meas> for Backend<'rbox> {
    type Color = i32;
    type Renderer = renderer::Renderer<'rbox>;
    type Measurer = measurer::Measurer;

    fn render(&mut self, node: &yoga_wrapper::Node) {
        let renderer = self.get_renderer();
        renderer.render(node);
        renderer.rustbox.present();
    }

    fn get_renderer(&mut self) -> &mut renderer::Renderer<'rbox> {
        &mut self.renderer
    }

    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas> {
        yoga_wrapper::Context::new(text, &self.measurer)
    }
}
