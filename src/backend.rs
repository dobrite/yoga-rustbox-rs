use rustbox;
use yoga;
use yoga_wrapper;

use yoga::Backend as B;
use yoga::Renders as R;

use measurer;
use renderer;

pub struct Backend<'r> {
    renderer: renderer::Renderer<'r>,
    measurer: measurer::Measurer,
}

impl<'r> Backend<'r> {
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

impl<'r, 'm> yoga::Backend<'m> for Backend<'r> {
    type Color = i32;
    type Renderer = renderer::Renderer<'r>;
    type Measurer = measurer::Measurer;

    fn render(&self, renderer: &renderer::Renderer, node: &yoga_wrapper::Node) {
        renderer.render(node)
    }

    fn create_context<'s>(&'m self, text: &'s str) -> yoga_wrapper::Context<'s, 'm> {
        yoga_wrapper::Context::new(text, &self.measurer)
    }
}
