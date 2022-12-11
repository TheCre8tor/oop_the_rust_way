use std::fmt::Debug;

#[derive(Debug)]
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

/* EXPLANATION:
 * If encapsulation is a required aspect for a language to
 * be considered object-oriented, then Rust meets that requirement.
 * The option to use pub or not for different parts of code enables
 * encapsulation of implementation details.
 * */

impl AverageCollection {
    pub fn new(list: Vec<i32>) -> AverageCollection {
        AverageCollection { list, average: 0.0 }
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
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
    }
}
