use rustbox;
use yoga;

use yoga::Renders;
use yoga::Renderable;

use builder;
use renderer;

pub struct Backend<'rbox> {
    renderer: renderer::Renderer<'rbox>,
}

impl<'rbox> Backend<'rbox> {
    pub fn new(rustbox: &rustbox::RustBox) -> Backend {
        Backend { renderer: renderer::Renderer::new(rustbox) }
    }
}

impl<'rbox, 'meas> yoga::Backend<'meas> for Backend<'rbox> {
    type Renderer = renderer::Renderer<'rbox>;

    fn get_renderer(&mut self) -> &mut renderer::Renderer<'rbox> {
        &mut self.renderer
    }
}
