fn main() {
    let guess_100 = Guess::new(100).value();
    println!("Guess 100 should be fine {guess_100}");

    let guess_200 = Guess::new(200).value();
    println!("Guess 200 should panic {guess_200}");
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
