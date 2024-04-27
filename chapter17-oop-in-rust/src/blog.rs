pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        let new_content = self.state.as_ref().unwrap().add_text(&self, text);
        self.content = new_content.to_string();
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(e) = self.state.take() {
            self.state = Some(e.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(e) = self.state.take() {
            self.state = Some(e.reject());
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self) // as_ref() 方法返回一个指向 state 字段的引用，而不是获取它的所有权，不能将 state 字段移动出 Post 实例
    }
    pub fn state(&self) -> &str {
        self.state.as_ref().unwrap().as_str()
    }
    pub fn draft(&self) -> &str {
        &self.content
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // 这里的self是Box<Self>类型，这会获取Box<Self>的所有权，从而消费掉旧的状态实例
    // 因为使用 take 方法将 state 中的值取出并留下了一个 None, 因为 rust 不允许结构体实例中存在值为空的字段。

    // 由于默认实现返回 self，无法在运行时确定返回值大小，所以不能使用默认实现
    // fn approve(self: Box<Self>) -> Box<dyn State>{
    //     self
    // }

    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // 默认实现，返回了空字符串
        ""
    }
    fn as_str(&self) -> &str;
    fn add_text(&self, post: &Post, text: &str) -> String {
        post.content.clone()
    }
}

#[derive(Debug)]
pub struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn as_str(&self) -> &str {
        "Draft"
    }

    fn add_text(&self, post: &Post, text: &str) -> String {
        format!("{}{}", &post.content, text)
    }
}

#[derive(Debug)]
pub struct PendingReview {
    approve_count: u32,
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approve_count < 1 {
            Box::new(PendingReview {
                approve_count: self.approve_count + 1,
            })
        } else {
            Box::new(Published {})
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn as_str(&self) -> &str {
        "PendingReview"
    }
}

#[derive(Debug)]
pub struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn as_str(&self) -> &str {
        "Published"
    }
}
