use rustbox;
use yoga;

pub struct Builder {
}

impl yoga::Builds<rustbox::Color> for Builder {
    fn view<'r>() -> yoga::View<'r, rustbox::Color> {
        yoga::View::new()
    }

    fn text<'t>(text: &'t str) -> yoga::Text<rustbox::Color> {
        yoga::Text::new(text)
    }
}
