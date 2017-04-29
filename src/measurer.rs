use yoga;

pub struct Measurer {}

impl yoga::Measures for Measurer {
    fn measure(&self, text: &str) -> yoga::Size {
        yoga::Size {
            width: text.len() as f32,
            height: 1.0,
        }
    }
}
