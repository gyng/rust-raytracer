use std::iter::repeat;
use std::io::Write;

pub mod export;
pub mod import;

pub fn print_progress(noun: &str, start_time: ::time::Timespec, done: usize, total: usize) {
    let remaining_jobs = total - done;
    let progress: f64 = 100f64 * done as f64 / total as f64;
    let current_time = ::time::get_time().sec;
    let time_per_job = (current_time - start_time.sec) as f64 / done as f64;
    let remaining_time = time_per_job * remaining_jobs as f64;

    print!("\r{} {}/{} complete\t{:.2}% [{}]",
           noun, done, total, progress,
           ::util::make_progress_bar(progress / 100.0, 20)
           );

    if remaining_jobs == 0 {
      println!(" (took {:.2} min)     ", (current_time - start_time.sec) as f64 / 60.0);
    } else {
      print!(" ETA {:.2} min           ", remaining_time / 60.0);
      ::std::io::stdout().flush().ok().expect("failed to flush io");
    }
}

fn make_progress_bar(ratio: f64, length: usize) -> String {
    let filled = (ratio * length as f64).round() as usize;
    let mut bar: String = repeat('|').take(filled).collect();

    for _ in 0..(length - filled) {
        bar.push('-');
    }

    bar
}
