use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    // create a progress bar with a length of 100
    let pb = ProgressBar::new(100);

    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
    )
    .expect("Failed to set the progress bar template")
    .progress_chars("#$-");

    pb.set_style(style);

    for i in 0..100 {
        pb.set_message(format!("Processing item {}", i+1));
        pb.inc(1);
        thread::sleep(Duration::from_millis(50)); // sleep for 50ms to simulate work
    }

    pb.finish_with_message("COMPLETED");
}