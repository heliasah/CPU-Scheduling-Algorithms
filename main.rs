use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Process {
    id: i32,
    arrival: i32,
    burst: i32,
    remaining: i32,
    start: i32,
    finish: i32,
}

impl Process {
    fn new(id: i32, arrival: i32, burst: i32) -> Self {
        Process {
            id,
            arrival,
            burst,
            remaining: burst,
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

    processes.sort_by_key(|p| p.arrival);

    let mut current_time = 0;
    let mut ready_queue = VecDeque::new();
    let mut out = Vec::new();
    let mut process_map = processes.clone();
    let mut active_process: Option<Process> = None;

    while !process_map.is_empty() || !ready_queue.is_empty() || active_process.is_some() {
        // Enqueue processes that have arrived by the current time
        while !process_map.is_empty() && process_map[0].arrival <= current_time {
            ready_queue.push_back(process_map.remove(0));
        }

        if active_process.is_none() && !ready_queue.is_empty() {
            // Start a new process if there is no active process
            let next_process = ready_queue
                .iter_mut()
                .min_by_key(|p| p.remaining)
                .unwrap()
                .clone();
            if next_process.start == -1 {
                active_process = Some(Process {
                    start: current_time,
                    ..next_process
                });
            } else {
                active_process = Some(next_process);
            }
            ready_queue.retain(|p| p.id != active_process.as_ref().unwrap().id);
        }

        if let Some(mut process) = active_process.take() {
            // Process the active process
            process.remaining -= 1;
            out.push(process.id);

            if process.remaining == 0 {
                process.finish = current_time + 1;
                for p in &mut processes {
                    if p.id == process.id {
                        *p = process.clone();
                    }
                }
                active_process = None;
            } else {
                active_process = Some(process);
            }
        }

        current_time += 1;
    }

    let mut timer = 0;
    let mut temp = out[0];
    out.remove(0);

    println!("Time {}: Task {} starts", timer, temp);

    timer += 1;
    for &it in &out {
        if temp != it {
            if let Some(p) = processes.iter().find(|p| p.id == temp) {
                if p.finish == timer {
                    println!("Time {}: Task {} finished", timer, temp);
                } else {
                    println!("Time {}: Task {} paused", timer, temp);
                }
            }
            temp = it;
            if let Some(p) = processes.iter().find(|p| p.id == it) {
                if p.start == timer {
                    println!("Time {}: Task {} starts", timer, it);
                } else {
                    println!("Time {}: Task {} resumes", timer, it);
                }
            }
        }
        timer += 1;
    }
    println!(
        "Time {}: Task {} finished, All tasks finished.",
        timer, temp
    );

    let mut sum_waiting_time = 0;
    let mut sum_turn_around_time = 0;

    for it in &processes {
        let waiting_time = it.finish - it.arrival - it.burst;
        let turnaround_time = it.finish - it.arrival;
        sum_waiting_time += waiting_time;
        sum_turn_around_time += turnaround_time;
    }

    let average_waiting_time = sum_waiting_time as f32 / processes.len() as f32;
    let average_turn_around_time = sum_turn_around_time as f32 / processes.len() as f32;

    println!("Average waiting time: {:.2}", average_waiting_time);
    println!("Average turnaround time: {:.2}", average_turn_around_time);

    Ok(())
}
