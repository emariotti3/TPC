use std::thread;
use std::sync::{Mutex, Arc, mpsc};
use std::time::Duration;

use network::message::{Message};

#[derive(Debug)]
pub struct Server {
	id: usize,
    rx: mpsc::Receiver<Message>,
	tx: mpsc::Sender<Message>,
    pub process_time: usize,
    observatories_senders: Vec<mpsc::Sender<Message>>,
}

impl Server {
	pub fn new(id: usize, process_time: usize) -> Server{ 
		let (tx, rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

		Server{id:id, rx:rx, tx:tx, process_time:process_time, observatories_senders: Vec::new()}
	}

	pub fn set_observatories_senders(&mut self, observatories_senders: Vec<mpsc::Sender<Message>>){
		self.observatories_senders = observatories_senders;
	}

	pub fn get_sender(&self) -> mpsc::Sender<Message>{
		return mpsc::Sender::clone(&self.tx);
	}

	pub fn run(&self,running: &Arc<Mutex<bool>>) {
        while {*running.lock().unwrap()}{
        	let valor_recibido = self.rx.recv().unwrap();
            println!("Server {} reciv foro id {}", self.id, valor_recibido.id_photo);

            thread::sleep(Duration::from_millis(1000*self.process_time as u64));

            self.observatories_senders[valor_recibido.id_observatory].send(valor_recibido).unwrap();
			// println!("Server {} send", self.id);
     	}
		println!("Goodbye from server {}", self.id);
	}
}
