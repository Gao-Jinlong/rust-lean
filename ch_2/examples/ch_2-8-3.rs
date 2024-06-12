trait Draw {
    fn draw(&self);
}
struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Button({}, {}, {})", self.width, self.height, self.label)
    }
}
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox({}, {}, {:?})",
            self.width, self.height, self.options
        )
    }
}

// 特征对象
fn draw(x: Box<dyn Draw>) {
    x.draw();
}
fn draw_without_box(x: &dyn Draw) {
    x.draw();
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };
    let select_box = SelectBox {
        width: 75,
        height: 30,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    };

    draw_without_box(&button);
    draw_without_box(&select_box);

    draw(Box::new(button));
    draw(Box::new(select_box));

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
