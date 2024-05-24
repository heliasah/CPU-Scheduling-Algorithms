use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Process {
    id: i32,
    arrival: i32,
    burst: i32,
    start: i32,
    finish: i32,
}

impl Process {
    fn new(id: i32, arrival: i32, burst: i32) -> Self {
        Process {
            id,
            arrival,
            burst,
            start: -1,
            finish: -1,
        }
    }
}

fn split(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() -> io::Result<()> {
    let path = Path::new("C:/Users/Helia/Downloads/rust/input.txt");
    let file = File::open(&path)?;

    let mut processes = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let vec = split(&line);
        let id = vec[0];
        let arrival = vec[1];
        let burst = vec[2];
        let process = Process::new(id, arrival, burst);
        processes.push(process);
    }

    let mut current_time = 0;

    // Sort processes based on arrival time (FCFS scheduling)
    processes.sort_by_key(|p| p.arrival);

    let mut out = Vec::new();

    for process in &mut processes {
        while current_time < process.arrival {
            current_time += 1;
        }
        process.start = current_time;
        out.push(process.id);
        current_time += process.burst;
        process.finish = current_time;
    }

    let mut timer = 0;
    let mut temp = out[0];
    out.remove(0);

    println!("Time {}: Task {} start", timer, temp);

    timer += processes[0].burst;
    for &it in &out {
        println!("Time {}: Task {} finished", timer, temp);
        temp = it;
        println!("Time {}: Task {} start", timer, it);
        timer += processes[it as usize - 1].burst;
    }
    println!("Time {}: Task {} finished All tasks finished", timer, temp);

    let mut sum_waiting_time = 0;
    let mut sum_turn_around_time = 0;

    for it in &processes {
        sum_waiting_time += it.start - it.arrival;
        sum_turn_around_time += it.finish - it.arrival;
    }

    let average_waiting_time = sum_waiting_time as f32 / processes.len() as f32;
    let average_turn_around_time = sum_turn_around_time as f32 / processes.len() as f32;

    println!("Average waiting time: {:.2}", average_waiting_time);
    println!("Average turn around time: {:.2}", average_turn_around_time);

    Ok(())
}
