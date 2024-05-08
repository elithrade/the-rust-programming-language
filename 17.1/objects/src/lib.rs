pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn default() -> Self {
        Self {
            list: vec![],
            average: -1.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut avg_collection = AveragedCollection::default();
        avg_collection.add(1);
        avg_collection.add(2);
        avg_collection.add(3);
        avg_collection.add(4);
        let result = avg_collection.average();
        assert_eq!(result, 2.5);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        let avg_collection = AveragedCollection::default();
        let result = avg_collection.average();
        assert_eq!(result, 0.0);
    }
}
