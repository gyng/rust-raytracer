use std::iter::repeat;
use std::num::Float;

pub mod export;
pub mod import;

pub fn print_progress(noun: &str, start_time: ::time::Timespec, done: uint, total: uint) {
    let remaining_jobs = total - done;
    let progress: f64 = 100f64 * done as f64 / total as f64;
    let current_time = ::time::get_time().sec;
    let time_per_job = (current_time - start_time.sec) as f64 / done as f64;
    let remaining_time = time_per_job * remaining_jobs as f64;

    print!("\r{} {}/{} complete\t{}% [{}]",
           noun, done, total,
           ::std::f64::to_str_exact(progress, 2),
           ::util::make_progress_bar(progress / 100.0, 20)
           );

    if remaining_jobs == 0 {
      println!(" (took {:.2} min)     ", (current_time - start_time.sec) as f64 / 60.0);
    } else {
      print!(" ETA {} min           ", ::std::f64::to_str_exact(remaining_time / 60.0, 2));
      ::std::io::stdio::flush();
    }
}

fn make_progress_bar(ratio: f64, length: uint) -> String {
    let filled = (ratio * length as f64).round() as uint;
    let mut bar: String = repeat('|').take(filled).collect();

    for _ in range(0, length - filled) {
        bar.push('-');
    }

    bar
}
