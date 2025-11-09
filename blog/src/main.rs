use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert!(post.content().is_empty());
    let post = post.request_review();
    assert!(post.content().is_empty());
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
