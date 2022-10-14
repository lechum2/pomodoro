use indicatif::ProgressBar;
use std::process::Command;
use std::{env, thread, time};

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
    //sleep 40m && notify-send -u critical 'Time is up!' -i stock_dialog-warning && paplay ~/Music/sounds/sms-alert-2-daniel_simon.wav
}
