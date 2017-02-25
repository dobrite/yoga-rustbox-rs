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
    fn create_context<'text>(&'meas self,
                             text: &'text str)
                             -> Box<yoga_wrapper::Context<'text, 'meas>> {
        Box::new(yoga_wrapper::Context::new(text, &self.measurer))
    }

    fn view<'r>(&self) -> yoga::View<'r, rustbox::Color> {
        yoga::View::new()
    }

    fn text<'text>(&'meas self, text: &'text str) -> yoga::Text<'text, 'meas, rustbox::Color> {
        yoga::Text::new(text, self.create_context(text))
    }
}
