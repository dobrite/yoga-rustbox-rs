use yoga_wrapper;

pub struct Measurer {}

impl yoga_wrapper::Measures for Measurer {
    fn measure(&self, text: &str) -> yoga_wrapper::Size {
        yoga_wrapper::Size {
            width: text.len() as f32,
            height: 1.0,
        }
    }
}
