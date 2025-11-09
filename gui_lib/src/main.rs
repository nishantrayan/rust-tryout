pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn average(&mut self) -> f64 {
        self.update_average();
        self.average
    }
}

fn main() {
    let mut collection = AveragedCollection {
        list: vec![1,2,3],
        average: 0.0,
    };
    println!("average = {}", collection.average());
}
