use rustbox;
use yoga;
use yoga_wrapper;

use measurer;

pub struct Builder {
    measurer: measurer::Measurer,
}

impl Builder {
    pub fn new() -> Self {
        Builder { measurer: measurer::Measurer {} }
    }
}

impl<'meas> yoga::Builds<'meas, rustbox::Color> for Builder {
    fn create_context<'text>(&'meas self, text: &'text str) -> yoga_wrapper::Context<'text, 'meas> {
        yoga_wrapper::Context::new(text, &self.measurer)
    }

    fn view<'r>(&self) -> yoga::View<'r, rustbox::Color> {
        yoga::View::new()
    }

    fn text<'t, 'a: 't>(&'a self, text: &'t str) -> yoga::Text<'t, rustbox::Color> {
        yoga::Text::new(text, &mut self.create_context(text))
    }
}
