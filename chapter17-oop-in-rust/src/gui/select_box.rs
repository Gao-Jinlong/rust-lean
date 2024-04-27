use super::Draw;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl SelectBox {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box with width: {}, height: {}, and options: {:?}",
            self.width, self.height, self.options
        );
    }
}
