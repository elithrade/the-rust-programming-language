use std::collections::HashMap;

fn main() {
    // Create.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Same key type, &str is not String.
    // scores.insert("Blue", 60);

    let mut new_scores = HashMap::new();
    new_scores.insert("Green", 10);
    new_scores.insert("Red", 50);

    // Accessing values.
    let team_name = String::from("Blue");
    // The get method returns an Option<&V>; if there’s no value for that key
    // in the hash map, get will return None. This program handles the Option
    // by calling copied to get an Option<i32> rather than an Option<&i32>,
    // then unwrap_or to set score to zero if scores doesn't have an entry for the key.
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterating.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{field_name}")

    // Inserting twice with the same key.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding only key not exists.
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value of the key.
        let count = map.entry(word).or_insert(0);
        // Dereferencing count so we can increment the value.
        *count += 1;
    }

    println!("{:?}", map);
}
