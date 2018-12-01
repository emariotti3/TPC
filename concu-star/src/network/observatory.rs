use std::{thread, time};
use std::sync::{Mutex, Arc, mpsc};
use std::collections::HashMap;

const SECONDS: usize = 0;
const QUADRANT_QTY: usize = 1;
const QUADRANTS_PER_SERVER: usize = 2;

use network::message::{Message};

// pub struct Sender {
// 	handle: std::thread::JoinHandle<i32>
// }

// pub struct Receiver {
// 	handle: std::thread::JoinHandle<i32>
// }

pub struct Observatory {
	id: usize,
    total_time: f64,
    events: f64,
	avg_time: Arc<Mutex<f64>>,
	quadrant_qty: usize,
	seconds: i32,
	quadrants_per_server: Vec<i32>,
	// sender: Sender,
	// receiver: Receiver,

	rx: mpsc::Receiver<Message>,
	tx: mpsc::Sender<Message>,
	servers_senders: Vec<mpsc::Sender<Message>>,
}


impl Observatory {
	pub fn new(id:usize, init_time: Arc<Mutex<f64>>, running: &Arc<Mutex<bool>>, line: &str) -> Observatory {
		let running_m_sender = Arc::clone(running);
        let running_m_receiver = Arc::clone(running);

		let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

		Observatory { id:id,
			rx:rx, tx:tx, servers_senders:Vec::new(),
			total_time:0.0, 
			events:0.0, 
			avg_time: init_time, 
			quadrant_qty:0, 
			seconds:0,
			quadrants_per_server:Vec::new(), 
			// sender: Sender::new(id, running_m_sender),
			// receiver: Receiver::new(id,running_m_receiver)
		}
	}

	pub fn set_servers_senders(&mut self, servers_senders: Vec<mpsc::Sender<Message>>){
		self.servers_senders = servers_senders;
	}

	pub fn get_sender(&self) -> mpsc::Sender<Message>{
		return mpsc::Sender::clone(&self.tx);
	}

	pub fn run(&mut self, running: &Arc<Mutex<bool>>) {
		let continue_running_server_processor = Arc::clone(running);

		let _id = self.id; // no le pudo pasar atributos  con self al hilo
		let _quadrants_per_server = self.quadrants_per_server.clone();
		let _servers_senders = self.servers_senders.clone();

		let server_sender = thread::spawn(move || {
			let mut id_message = 0;
			while {*continue_running_server_processor.lock().unwrap()} {
				let now = time::Instant::now();
				let _message = Message{id_observatory: _id, id_photo: id_message, start_time: now };
				for server in &_quadrants_per_server {
					let message = _message.clone();
                	println!("Observatory {} send to server {}", _id, *server);
                	_servers_senders[*server as usize].send(message).unwrap();
					thread::sleep(time::Duration::from_millis(1000));	
        		}
				id_message += 1;
			}
			println!("corto el hilo sender {}", _id);
    	});
		

		let mut sending_messages: HashMap<usize, usize>  = HashMap::new();

		while {*running.lock().unwrap()} {
            let valor_recibido = self.rx.recv().unwrap();
			// println!("Observatory {} reciv foto id {}", _id, valor_recibido.id_photo);
			//HACER EL CHEQUEO CON EL VALOR RECIBIDO
			self.process_new_messege(&valor_recibido, &mut sending_messages);

        }
		server_sender.join().unwrap();   
		println!("Goodbye from observatory run {}", _id);
	}

	fn process_new_messege(&mut self,message: &Message ,messages: &mut HashMap<usize, usize> ){
		let _id_photo = message.id_photo;
		let mut quadrants_count = *messages.entry(_id_photo).or_insert(0);
		quadrants_count += 1;
		if quadrants_count == self.quadrant_qty {
			println!("Observatory {} termino de procesar la foto {} en tiempo {}", self.id , _id_photo, message.start_time.elapsed().as_secs());
			return;
		}	
		println!("Observatory {} recibio un nuevo cuadrande{} de foto {}", self.id, quadrants_count, _id_photo);

		messages.insert(_id_photo, quadrants_count);
		// println!(" dic  {:?}",messages);
	}

	// pub fn graceful_quit(self) {
	// 	//TODO:finish sending
	// 	self.sender.quit();
	// 	//TODO:finish receiving
	// 	self.receiver.quit();
	// }

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

// impl Sender {
// 	fn new(id: usize, running_m: Arc<Mutex<bool>>) -> Sender {
// 		Sender {
// 			handle: thread::spawn(move || {
// 				let mut continue_running = true;
// 				while continue_running {
// 		            println!("observatory {} started sender!", id);
// 		            let ten = time::Duration::from_millis(10000);
// 					let now = time::Instant::now();

// 					thread::sleep(ten);		            
// 		            {
// 						continue_running = *running_m.lock().unwrap()
// 					}
// 		        }
// 		        println!("Goodbye from observatory sender {}", id);
// 		        return 0;
// 	        })
// 		}
// 	}

// 	pub fn quit(self) {
// 		self.handle.join().unwrap();
// 	}
// }

// impl Receiver {
// 	fn new(id: usize, running_m: Arc<Mutex<bool>>) -> Receiver {
// 		Receiver {
// 			handle: thread::spawn(move || {
// 				let mut continue_running = true;
// 				while continue_running {
// 		            println!("observatory {} started receiver!", id);
// 		            let ten = time::Duration::from_millis(10000);
// 					let now = time::Instant::now();

// 					thread::sleep(ten);
// 		            {
// 						continue_running = *running_m.lock().unwrap()
// 					}
// 		        }
// 		        println!("Goodbye from observatory receiver {}", id);
// 		        return 0;
// 	        })
// 		}
// 	}

// 	pub fn quit(self) {
// 		self.handle.join().unwrap();
// 	}
// }
