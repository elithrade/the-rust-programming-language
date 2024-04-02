struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax to set a new email value for a User instance but to use the
    // rest of the values from user1.
    // we can no longer use user1 as a whole after creating user2 because the String in the
    // username field of user1 was moved into user2.
    // If we had given user2 new String values for both email and username, and thus only used the
    // the active and sign_in_count values from user1, then user1 would still be valid after creating user2
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    assert_eq!(user2.active, user3.active);
    assert_eq!(user2.username, user3.username);
    assert_eq!(user2.email, user3.email);
    assert_eq!(user2.sign_in_count, user3.sign_in_count);
}
