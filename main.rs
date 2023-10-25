use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use statistics::StatisticsError;

fn denirostats(file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut scores = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let score = line.split(',').nth(1).unwrap().parse::<f64>()?;
        scores.push(score);
    }
    let mean_score = scores.iter().sum::<f64>() / scores.len() as f64;
    let median_score = {
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted_scores.len() / 2;
        if sorted_scores.len() % 2 == 0 {
            (sorted_scores[mid - 1] + sorted_scores[mid]) / 2.0
        } else {
            sorted_scores[mid]
        }
    };
    let std_dev = match statistics::standard_deviation(&scores, None) {
        Ok(std_dev) => std_dev,
        Err(StatisticsError::TooFewSamples) => 0.0,
        Err(e) => return Err(Box::new(e)),
    };
    println!("Mean Score: {:.2}", mean_score);
    println!("Median Score: {:.2}", median_score);
    println!("Standard Deviation of Scores: {:.2}", std_dev);
    Ok(())
}

fn main() {
    let initial_memory = mem_info().unwrap().avail;

    let start_time = Instant::now();
    denirostats("deniro.csv").unwrap();

    let duration = start_time.elapsed();
    let final_memory = mem_info().unwrap().avail;
    let memory_usage = initial_memory as f64 - final_memory as f64;

    println!("\nRuntime: {:.2?}", duration);
    println!("Memory Usage: {:.2} MB", memory_usage / 1024.0);

    Ok(())
}
