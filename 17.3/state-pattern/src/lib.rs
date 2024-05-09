// Public Post struct that holds some content.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// State trait defines the behaviour shared by different post states.
trait State {}

// The default state of a Post.
struct Draft {}

impl State for Draft {}
