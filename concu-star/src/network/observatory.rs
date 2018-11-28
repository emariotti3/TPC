use std::sync::{Mutex, Arc};

pub struct Observatory {
	id:i32,
    total_time: f64,
    events: f64,
	avg_time: Arc<Mutex<f64>>,
	quadrant_qty: i32,
	seconds: i32,
	quadrants_per_server: Vec<i32>
}

impl Observatory {
	pub fn new(id:i32, init_time: Arc<Mutex<f64>>, line: &str) -> Observatory{
		Observatory{total_time:0.0, events:0.0, avg_time: init_time, quadrant_qty:0,seconds:0,quadrants_per_server:Vec::new()};
		parse_line(line);
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

	fn parse_line(String line){
		//parseo linea y meto los datos en el struct
		let params: Vec<&str> = line.split(" ").collect();
		self.quadrant_qty = params[1]; //Pasar a constantes
	}


}