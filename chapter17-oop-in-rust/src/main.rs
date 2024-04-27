mod blog;
mod gui;

use crate::gui::{Button, Screen, SelectBox};
use blog::Post;

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Cancel"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();

    println!("==============================");
    blogMain();
}

fn blogMain() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("draft = {:?}", post.draft());

    post.add_text(", and it was delicious");
    assert_eq!("", post.content());
    assert_eq!(
        "I ate a salad for lunch today, and it was delicious",
        post.draft()
    );
    println!("draft = {:?}", post.draft());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());
    println!("post state = {:?}", post.state());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(
        "I ate a salad for lunch today, and it was delicious",
        post.content()
    );
}
