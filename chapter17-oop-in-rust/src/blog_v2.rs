// blog.rs 中使用面向对象的 状态模式 来实现一个博客的状态转换，但是有非常多的冗余代码
// blog_v2.rs 使用更适合 Rust 的方式来实现相同的功能
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        // 注意这里的 self 转移了所有权，从而实现了销毁 DraftPost
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
