use std::{thread, time};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::sync::{Mutex, Arc, mpsc};
use std::collections::HashMap;

const SECONDS: usize = 0;
const QUADRANT_QTY: usize = 1;
const QUADRANTS_PER_SERVER: usize = 2;

use network::message::{Message};

pub struct Observatory {
	id: usize,
    total_time: f64,
    events: f64,
	avg_time: Arc<Mutex<f64>>,
	quadrant_qty: usize,
	seconds: i32,
	quadrants_per_server: Vec<i32>,
	rx: mpsc::Receiver<Message>,
	tx: mpsc::Sender<Message>,
	servers_senders: Vec<mpsc::Sender<Message>>
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
			quadrants_per_server:Vec::new()
		}
	}

	pub fn set_servers_senders(&mut self, servers_senders: Vec<mpsc::Sender<Message>>){
		self.servers_senders = servers_senders;
	}

	pub fn get_sender(&self) -> mpsc::Sender<Message>{
		return mpsc::Sender::clone(&self.tx);
	}

	pub fn run(&mut self, running: &Arc<Mutex<bool>>) -> (JoinHandle<i32>, JoinHandle<i32>) {
		let continue_running_sender = Arc::clone(running);
		let continue_running_receiver = Arc::clone(running);

		let _id = self.id; // no le pudo pasar atributos  con self al hilo
		let _quadrants_per_server = self.quadrants_per_server.clone();
		let _servers_senders = self.servers_senders.clone();

		println!("Observatory {} is up!", self.id);

		//Spawn sender thread
		let sender = thread::spawn(move || {
			let mut id_message = 0;

			while {*continue_running_sender.lock().unwrap()} {
				let now = time::Instant::now();
				let _message = Message{id_observatory: _id, id_photo: id_message, start_time: now };
				for server in &_quadrants_per_server {
					//let message = _message.clone();
                	println!("Observatory {} send to server {}", _id, *server);
                	//_servers_senders[*server as usize].send(message).unwrap();
					thread::sleep(time::Duration::from_millis(1000*5));	
        		}
				id_message += 1;
			}
			println!("Observatory {}: sender thread is down!", _id);
			return 0;
    	});

		//Spawn receiver thread		
		let receiver = thread::spawn(move || {
			let mut sending_messages: HashMap<usize, usize>  = HashMap::new();

			while {*continue_running_receiver.lock().unwrap()} {
	            //let valor_recibido = self.rx.recv().unwrap();
				//println!("Observatory {} reciv foto id {}", _id, valor_recibido.id_photo);
				//HACER EL CHEQUEO CON EL VALOR RECIBIDO
				//self.process_new_messege(&valor_recibido, &mut sending_messages);
				println!("Observatory receiver {} is running!", _id);
				thread::sleep(time::Duration::from_millis(1000*5));

	        }
			println!("Observatory {}: receiver thread is down!", _id);
	        return 0;
    	});

		return (sender, receiver);
	}

	fn process_new_messege(&mut self,message: &Message ,messages: &mut HashMap<usize, usize> ){
		let _id_photo = message.id_photo;
		let mut quadrants_count = *messages.entry(_id_photo).or_insert(0);
		quadrants_count += 1;
		if quadrants_count == self.quadrant_qty {
			//Add image process time to total:
			self.total_time += message.start_time.elapsed().as_secs() as f64;
			self.events += 1.0;
			//Update global avg variable:
			self.update_avg_time();
			println!("Observatory {} termino de procesar la foto {} en tiempo {}", self.id , _id_photo, message.start_time.elapsed().as_secs());
			return;
		}	
		println!("Observatory {} recibio un nuevo cuadrande{} de foto {}", self.id, quadrants_count, _id_photo);

		messages.insert(_id_photo, quadrants_count);
		// println!(" dic  {:?}",messages);
	}

	fn update_avg_time(&mut self) {
        let mut time = self.avg_time.lock().unwrap();
        *time = self.total_time / self.events;
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


