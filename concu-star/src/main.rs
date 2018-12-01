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
    let mut avg_times = Vec::new();
    let running = Arc::new(Mutex::new(true));
    let file = File::open("config.txt").expect("Configuration file not found!");
    let mut observatory_count = 0;

    let cant_servidores = 3;
    let mut children = vec![];
    let mut servers: Vec<Server> = Vec::new();
    let mut senders_observatorios: Vec<mpsc::Sender<Message>> = Vec::new();
    let mut senders_servers: Vec<mpsc::Sender<Message>> = Vec::new();
    
    for _s in 0..cant_servidores {
        let mut server = Server::new(_s,_s*3);
        senders_servers.push(server.get_sender());
        servers.push(server);
    }        

    for line in BufReader::new(file).lines() {
        avg_times.push(Arc::new(Mutex::new(0.0)));
        let avg_time = Arc::clone(&avg_times[observatory_count]);

        //Create observatory
        let mut obs = Observatory::new(observatory_count, avg_time, &running, "");
        //Initialize observatory
        obs.parse_line(&line.unwrap());
        senders_observatorios.push(obs.get_sender());
        obs.set_servers_senders(senders_servers.clone());
        //Run observatory
        let _running = Arc::clone(&running);
        let handles = obs.run(&_running);
        //Save observatory threads for join on 'q'
        children.push(handles.0);
        children.push(handles.1);
        
        observatory_count += 1;
        println!("Observatory count {}", observatory_count);
    }

    for mut server in servers {
        server.set_observatories_senders(senders_observatorios.clone());
        let _running = Arc::clone(&running);
        children.push(thread::spawn(move || { 
            server.run(&_running);
            return 0;
        }));
    } 

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line!");
        let user_input: String = line.trim().parse().expect("Wanted a valid string!");

        //Read from stdin until user enters 'q' to exit
        if user_input.to_lowercase() == "q" {
            println!("Received q! Now executing graceful quit...");
            {
                let mut running_val = running.lock().unwrap();
                *running_val = false;
            }
            for mut handle in children {
                handle.join().unwrap();
            }
            return;
        }

        //or if user enters a number, print average time
        //for the observatory with that number
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
}
