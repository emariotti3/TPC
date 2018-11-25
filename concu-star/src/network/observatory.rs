use std::sync::{Mutex, Arc};

pub struct Observatory {
    list: Vec<f64>,
	avg_time: Arc<Mutex<f64>>
}

impl Observatory {
	pub fn new(init_time: Arc<Mutex<f64>>) -> Observatory{
		Observatory{list:Vec::new(), avg_time: init_time}
	}

	pub fn get_avg_time(&self) -> f64 {
		let time = self.avg_time.lock().unwrap();
		return *time;
	}

    pub fn add(&mut self, value: f64) {
        self.list.push(value);
        self.update_average();
    }

	fn update_average(&mut self) {
        let total: f64 = self.list.iter().sum();
        let mut time = self.avg_time.lock().unwrap();
        *time = total as f64 / self.list.len() as f64;
	}


}