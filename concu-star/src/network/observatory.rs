use std::sync::{Mutex, Arc};

const SECONDS: usize = 0;
const QUADRANT_QTY: usize = 1;
const QUADRANTS_PER_SERVER: usize = 2;

pub struct Observatory {
    total_time: f64,
    events: f64,
	avg_time: Arc<Mutex<f64>>,
	quadrant_qty: i32,
	seconds: i32,
	quadrants_per_server: Vec<i32>
}

impl Observatory {
	pub fn new(init_time: Arc<Mutex<f64>>) -> Observatory{
		Observatory{total_time:0.0, events:0.0, avg_time: init_time, quadrant_qty:0,seconds:0,quadrants_per_server:Vec::new()}
	}

	pub fn get_avg_time(&self) -> f64 {
		let time = self.avg_time.lock().unwrap();
		return *time;
	}

    pub fn add(&mut self, value: f64) {
        self.total_time += value;
    }

	fn update_average(&mut self) {
        let mut time = self.avg_time.lock().unwrap();
        *time = self.total_time as f64 / self.events as f64;
	}

	pub fn parse_line(&mut self, line: &str){
		let params: Vec<&str> = line.split(" ").collect();
	
		self.seconds = params[SECONDS].parse().unwrap();
		self.quadrant_qty = params[QUADRANT_QTY].parse().unwrap();
		for i in QUADRANTS_PER_SERVER..params.len(){
			self.quadrants_per_server.push(params[i].parse().unwrap());
		};
	}


}