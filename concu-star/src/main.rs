use std::io;
use std::thread;
use std::usize;
use std::sync::{Mutex, Arc};

mod network;
use network::observatory::{Observatory};

fn main() {
    let observatory_count = 3;
    let mut observatories = Vec::new();
    let mut avg_times = Vec::new();
    let mut running = Arc::new(Mutex::new(true));

    for _ in 0..observatory_count {
        avg_times.push(Arc::new(Mutex::new(0.0)));
    }

    for j in 0..observatory_count {
        let avg_time = Arc::clone(&avg_times[j]);
        let mut obs = Observatory::new(j, avg_time, &running, "");
        observatories.push(obs);
    }

    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        let user_input: String = line
            .trim()
            .parse()
            .expect("Wanted a valid string!");

        if user_input.to_lowercase() == "q" {
            {
                let mut running_val = running.lock().unwrap();
                *running_val = false;
            }
            for mut obs in observatories {
                obs.graceful_quit();
            }
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
}
