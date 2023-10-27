use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use sys_info::mem_info;

fn denirostats(file: &str) -> (f64, f64) {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let scores: Vec<f64> = reader
        .lines()
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let score = line.split(',').nth(1).unwrap().parse::<f64>().unwrap();
            score
        })
        .collect();

    let mean_score = scores.iter().sum::<f64>() / scores.len() as f64;

    let mut sorted_scores = scores.clone();
    sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median_score = if sorted_scores.len() % 2 == 0 {
        let mid = sorted_scores.len() / 2;
        (sorted_scores[mid - 1] + sorted_scores[mid]) / 2.0
    } else {
        sorted_scores[sorted_scores.len() / 2]
    };

    (mean_score, median_score)
}

fn main() {
    let initial_memory = mem_info().unwrap().avail;
    let start_time = Instant::now();
    let (mean_score, median_score) = denirostats("deniro.csv");
    println!("Mean Score: {:.2}", mean_score);
    println!("Median Score: {:.2}", median_score);
    
    let duration = start_time.elapsed();
    let final_memory = mem_info().unwrap().avail;
    let memory_usage = initial_memory as f64 - final_memory as f64;

    println!("\nRuntime: {:.2?}", duration);
    println!("Memory Usage: {:.2} MB", memory_usage / 1024.0);
}
