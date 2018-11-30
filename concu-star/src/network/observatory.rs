use std::{thread, time};
use std::sync::{Mutex, Arc};

const SECONDS: usize = 0;
const QUADRANT_QTY: usize = 1;
const QUADRANTS_PER_SERVER: usize = 2;

pub struct Sender {
	handle: std::thread::JoinHandle<i32>
}

pub struct Receiver {
	handle: std::thread::JoinHandle<i32>
}

pub struct Observatory {
	id: usize,
    total_time: f64,
    events: f64,
	avg_time: Arc<Mutex<f64>>,
	quadrant_qty: i32,
	seconds: i32,
	quadrants_per_server: Vec<i32>,
	sender: Sender,
	receiver: Receiver
}


impl Observatory {
	pub fn new(id:usize, init_time: Arc<Mutex<f64>>, running: &Arc<Mutex<bool>>, line: &str) -> Observatory {
		let running_m_sender = Arc::clone(running);
        let running_m_receiver = Arc::clone(running);
		Observatory { id:id,
			total_time:0.0, 
			events:0.0, 
			avg_time: init_time, 
			quadrant_qty:0, 
			seconds:0,
			quadrants_per_server:Vec::new(), 
			sender: Sender::new(id, running_m_sender),
			receiver: Receiver::new(id,running_m_receiver)
		}
	}

	pub fn graceful_quit(self) {
		//TODO:finish sending
		self.sender.quit();
		//TODO:finish receiving
		self.receiver.quit();
	}

	pub fn get_avg_time(&mut self) -> f64 {
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

impl Sender {
	fn new(id: usize, running_m: Arc<Mutex<bool>>) -> Sender {
		Sender {
			handle: thread::spawn(move || {
				let mut continue_running = true;
				while(continue_running) {
		            println!("observatory {} started sender!", id);
		            let ten = time::Duration::from_millis(10000);
					let now = time::Instant::now();

					thread::sleep(ten);		            
		            {
						continue_running = *running_m.lock().unwrap()
					}
		        }
		        println!("Goodbye from observatory sender {}", id);
		        return 0;
	        })
		}
	}

	pub fn quit(self) {
		self.handle.join().unwrap();
	}
}

impl Receiver {
	fn new(id: usize, running_m: Arc<Mutex<bool>>) -> Receiver {
		Receiver {
			handle: thread::spawn(move || {
				let mut continue_running = true;
				while(continue_running) {
		            println!("observatory {} started receiver!", id);
		            let ten = time::Duration::from_millis(10000);
					let now = time::Instant::now();

					thread::sleep(ten);
		            {
						continue_running = *running_m.lock().unwrap()
					}
		        }
		        println!("Goodbye from observatory receiver {}", id);
		        return 0;
	        })
		}
	}

	pub fn quit(self) {
		self.handle.join().unwrap();
	}
}
