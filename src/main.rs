use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Timelike};

fn calculate_year_progress(now: DateTime<Local>) -> Option<f64> {
    let start_of_year_naive = NaiveDate::from_ymd_opt(now.year(), 1, 1)?.and_hms_opt(0, 0, 0)?;
    let end_of_year_naive = NaiveDate::from_ymd_opt(now.year(), 12, 31)?.and_hms_opt(23, 59, 59)?;
    let start_of_year = Local.from_local_datetime(&start_of_year_naive).single()?;
    let end_of_year = Local.from_local_datetime(&end_of_year_naive).single()?;
    let duration_of_year = end_of_year
        .signed_duration_since(start_of_year)
        .num_seconds() as f64;
    let elapsed_this_year = now.signed_duration_since(start_of_year).num_seconds() as f64;
    Some((elapsed_this_year / duration_of_year) * 100.0)
}

fn print_progress_bar(percentage: f64, bar_length: usize) {
    let completed_length = (percentage / 100.0 * bar_length as f64).round() as usize;
    let uncompleted_length = bar_length - completed_length;
    let progress_bar = "#".repeat(completed_length) + &".".repeat(uncompleted_length);
    println!("[{}] {:.2}%", progress_bar, percentage);
}

fn main() {
    let now = Local::now();
    match calculate_year_progress(now) {
        Some(percentage) => {
            println!(
                "{:04}/{:02}/{:02} {:02}:{:02}:{:02}",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            print_progress_bar(percentage, 20);
        }
        None => println!("Could not calculate the progress of the year."),
    }
}
