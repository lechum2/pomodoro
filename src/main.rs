use chrono::Local;
use dirs;
use indicatif::{style::ProgressStyle, ProgressBar};
use std::{env, process::Command, thread, time};

fn main() {
    println!("Pomodoro started at: {}", Local::now().format("%H:%M"));
    let args: Vec<String> = env::args().collect();
    let minutes = &args[1].parse::<u64>().unwrap();
    let seconds = minutes * 60;

    let bar = ProgressBar::new(seconds);
    let style = ProgressStyle::with_template("{elapsed} {wide_bar} {eta}")
        .unwrap()
        .progress_chars("##-");
    bar.set_style(style);

    let one_sec = time::Duration::from_secs(1);
    for _ in 0..seconds {
        bar.inc(1);
        thread::sleep(one_sec);
    }
    bar.finish();

    Command::new("notify-send")
        .args([
            "-u",
            "critical",
            "'Time is up!'",
            "-i",
            "stock_dialog-warning",
        ])
        .spawn()
        .expect("error on nofify");

    let mut sound_path = dirs::home_dir().expect("could not resolve home dir");
    sound_path.push("Music/sounds/sms-alert-2-daniel_simon.wav");
    let sound_path_str = sound_path.to_str().expect("to string error");
    Command::new("paplay")
        .arg(sound_path_str)
        .spawn()
        .expect("error on sonund");
    println!("Pomodoro finished at: {}", Local::now().format("%H:%M"));
}
