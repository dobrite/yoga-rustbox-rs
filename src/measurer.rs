use yoga_wrapper;
use yoga_wrapper::Measures;

pub struct Measurer {}

impl Measures for Measurer {
    fn measure(&self, text: &str) -> yoga_wrapper::Size {
        yoga_wrapper::Size {
            width: text.len() as f32,
            height: 1.0,
        }
    }
}
