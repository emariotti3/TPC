use std::io;
use std::thread;
use std::usize;
use std::sync::{Mutex, Arc, mpsc};
use std::io::{BufReader,BufRead};
use std::fs::File;

mod network;
use network::observatory::{Observatory};
use network::server::{Server};
use network::message::{Message};

fn main() {
    let observatory_count = 3;
    // let mut observatories: Vec<Observatory> = Vec::new();
    let mut avg_times = Vec::new();
    let running = Arc::new(Mutex::new(true));
    let file = File::open("src/hola.txt").expect("file not found");;
    let mut j = 0;

    let cant_servidores = 3;
    let mut children = vec![];
    let mut servers: Vec<Server> = Vec::new();
    let mut senders_observatorios: Vec<mpsc::Sender<Message>> = Vec::new();
    let mut senders_servers: Vec<mpsc::Sender<Message>> = Vec::new();
    
    for _s in 0..cant_servidores {
        let mut server = Server::new(_s,2);
        senders_servers.push(server.get_sender());
        servers.push(server);
    }        

    for line in BufReader::new(file).lines() {
        avg_times.push(Arc::new(Mutex::new(0.0)));
        let avg_time = Arc::clone(&avg_times[j]);

        let mut obs = Observatory::new(j, avg_time, &running, "");
        obs.parse_line(&line.unwrap());
        obs.add(j as f64);
        senders_observatorios.push(obs.get_sender());
        obs.set_servers_senders(senders_servers.clone());
        // println!("observatory {} : {}", j, obs.get_avg_time());

        // observatories.push(obs);
        let _running = Arc::clone(&running);
        children.push(thread::spawn(move || { 
            obs.run(&_running);
        }));

        j += 1;
    }

    for mut server in servers {
        server.set_observatories_senders(senders_observatorios.clone());
        let _running = Arc::clone(&running);
        children.push(thread::spawn(move || { 
            server.run(&_running);
        }));
    } 

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line!");
        let user_input: String = line.trim().parse().expect("Wanted a valid string!");

        if user_input.to_lowercase() == "q" {
            {
                let mut running_val = running.lock().unwrap();
                *running_val = false;
            }
            // for mut obs in observatories {
            //     obs.graceful_quit();
            // }
            return;
        }

        let input = user_input.parse::<usize>();
        match input {
            Ok(number) => { 
                if number < observatory_count {
                    println!("Average time per image for observatory {} is: {}sec.", number ,*avg_times[number].lock().unwrap());
                }else{
                    println!("Please enter a valid observatory number! [0 : {}] or 'q' to exit!", observatory_count-1);
                }
            },
            Err(e) => println!("Wanted a positive number! ({})", e), 
        }  

    }
    // for child in children {
    //     child.join().unwrap();
    // }
}
