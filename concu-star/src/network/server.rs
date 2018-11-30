use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug)]
pub struct Server {
	id: usize,
    rx: mpsc::Receiver<i32>,
	tx: mpsc::Sender<i32>,
    pub process_time: usize,
    observatories_senders: Vec<mpsc::Sender<i32>>,
}

impl Server {
	pub fn new(id: usize, process_time: usize) -> Server{ 
		let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

		Server{id:id, rx:rx, tx:tx, process_time:process_time, observatories_senders: Vec::new()}
	}

	pub fn set_observatories_senders(&mut self, observatories_senders: Vec<mpsc::Sender<i32>>){
		self.observatories_senders = observatories_senders;
	}

	pub fn get_sender(&self) -> mpsc::Sender<i32>{
		return mpsc::Sender::clone(&self.tx);
	}

	pub fn run(&self) {
		// let mut server_processor = thread::spawn(move || { 
            loop {
                let valor_recibido = self.rx.recv().unwrap();
                println!("server {} recibio {}", self.id, valor_recibido);

                // println!("server {} va a dormir {} ", self.id, self.process_time);
                thread::sleep(Duration::from_millis(self.process_time as u64));

                self.observatories_senders[valor_recibido as usize].send(self.id as i32).unwrap();
				println!("server {} envio {}", self.id, valor_recibido);

            }
        // });

		// while(not grafullquit){
		// 	// no se
		// }
		// server_processor.join().unwrap();

	}
}
