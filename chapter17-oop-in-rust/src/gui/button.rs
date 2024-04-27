use super::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Button {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn label(&self) -> &String {
        &self.label
    }
}
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button with width: {}, height: {}, and label: {}",
            self.width, self.height, self.label
        );
    }
}
