use std::io;
use std::thread;
use std::usize;
use std::sync::{Mutex, Arc};
use std::io::{BufReader,BufRead};
use std::fs::File;

mod network;
use network::observatory::{Observatory};

fn main() {
    let observatory_count = 10;
    let mut handles = Vec::new();
    let mut avg_times = Vec::new();
    let file = File::open("hola.txt").expect("file not found");;
    let mut j = 0;

    for line in BufReader::new(file).lines() {
        avg_times.push(Arc::new(Mutex::new(0.0)));
        let avg_time = Arc::clone(&avg_times[j]);

        let handle = thread::spawn(move || {
            
            let mut obs = Observatory::new(avg_time);
            obs.parse_line(&line.unwrap());
            obs.add(j as f64);
            println!("observatory {} : {}", j, obs.get_avg_time());
        });
        handles.push(handle);
        j += 1;
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
            for handle in handles {
                handle.join().unwrap();
            }
            println!("Goodbye!");
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
