use indicatif::ProgressBar;
use std::process::Command;
use std::{env, thread, time};
extern crate dirs;

fn main() {
    println!("Pomodoro");
    let args: Vec<String> = env::args().collect();
    let minutes = &args[1].parse::<u64>().unwrap();
    let seconds = minutes * 60;

    let bar = ProgressBar::new(seconds);
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
}
