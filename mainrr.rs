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
    fn set_start(&mut self, start: i32) {
        self.start = start;
    }

    fn set_finish(&mut self, finish: i32) {
        self.finish = finish;
    }
}

fn find(processes: &mut Vec<Process>, id: i32) -> Option<&mut Process> {
    processes.iter_mut().find(|p| p.id == id)
}

fn split(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() -> io::Result<()> {
    let quantum_time = 2;

    let path = Path::new("C:/Users/Helia/Downloads/rust/input.txt");
    let file = File::open(&path)?;

    let mut processes = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let vec = split(&line);
        let id = vec[0];
        let arrival = vec[1];
        let burst = vec[2];
        let process = Process {
            id,
            arrival,
            burst,
            remaining: burst,
            start: -1,
            finish: -1,
        };
        processes.push(process);
    }

    let mut current_time = 0;
    let mut ready_queue = VecDeque::new();
    let mut out = Vec::new();

    loop {
        for p in &processes {
            if p.arrival == current_time {
                ready_queue.push_back(p.clone());
                break;
            }
        }

        if let Some(front) = ready_queue.front_mut() {
            if front.remaining > 0 {
                if find(&mut processes, front.id).unwrap().start == -1 {
                    find(&mut processes, front.id)
                        .unwrap()
                        .set_start(current_time);
                }
                out.push(front.id);
                front.remaining -= 1;
            }

            if front.remaining == 0 {
                find(&mut processes, front.id)
                    .unwrap()
                    .set_finish(current_time + 1);
                ready_queue.pop_front();
            } else if (front.burst - front.remaining) % quantum_time == 0 {
                if let Some(process) = ready_queue.pop_front() {
                    ready_queue.push_back(process);
                }
            }
        }

        if ready_queue.is_empty() {
            break;
        }

        current_time += 1;
    }

    let mut timer = 0;
    let mut temp = out[0];
    out.remove(0);

    println!("Time {}: Task {} start", timer, temp);

    timer += 1;
    for &it in &out {
        if temp != it {
            if find(&mut processes, it).unwrap().finish == timer {
                println!("Time {}: Task {} finished,", timer, temp);
            } else {
                println!("Time {}: Task {} paused,", timer, temp);
            }
            temp = it;
            if find(&mut processes, it).unwrap().start == timer {
                println!("Time {}: Task {} start", timer, it);
            } else {
                println!("Time {}: Task {} resumes", timer, it);
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
        sum_waiting_time += it.finish - it.arrival - it.burst;
        sum_turn_around_time += it.finish - it.arrival;
    }

    let average_waiting_time = sum_waiting_time as f32 / processes.len() as f32;
    let average_turn_around_time = sum_turn_around_time as f32 / processes.len() as f32;

    println!("Average waiting time: {}", average_waiting_time);
    println!("Average turn around time: {}", average_turn_around_time);

    Ok(())
}
