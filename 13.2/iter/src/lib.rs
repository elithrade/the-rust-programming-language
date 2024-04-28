pub fn iter_v1() {
    let v1 = [1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        iter_v1();
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = [1, 2, 3];

        // Calling next on an iterator changes the internal state that the iterator uses to keep
        // track of where it is in the sequence. Each call to next eats up an item from the
        // iterator.
        //
        // We don't need to mut in iter_v1 when we use iterator in for loop because the loop takes
        // ownership and makes it mutable behind the scenes.
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        assert_eq!(v1.len(), 3);

        println!("{:?}", v1)
    }

    #[test]
    fn iterator_sum() {
        let v1 = [1, 2, 3];

        let v1_iter = v1.iter();

        // sum() consumes the iterator by taking over the iterator and repeatedly calling next.
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
