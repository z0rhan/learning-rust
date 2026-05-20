use oop_features::gui::{Button, Draw, Screen};
use oop_features::blog_idiomatic::Post;

#[derive(Debug)]
struct TextLabel {
    label: String,
    value: String,
}

impl Draw for TextLabel {
    fn draw(&self) {
        println!("Drawing text-label: {:?}", self)
    }
}

fn main() {
    polymorphism();
    state_object_pattern();
}

fn polymorphism() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("Login")
            }),
            Box::new(TextLabel {
                label: String::from("Year"),
                value: String::from("1990")
            })
        ]
    };

    screen.run();
}

fn state_object_pattern() {
    let mut post = Post::new();
    post.add_text("This is my first post");
    let post = post.request_review();
    let post = post.approve();

    assert_eq!(
        "This is my first post",
        post.content()
    )
}
