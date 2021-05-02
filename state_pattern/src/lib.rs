pub mod blog {
    pub struct Post {
        content: String,
    }

    impl Post {
        #[allow(clippy::clippy::clippy::new_ret_no_self)]
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
}