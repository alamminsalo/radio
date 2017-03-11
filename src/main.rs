extern crate notify_rust;
extern crate chrono;

use std::env;

mod audiostream;

fn current_timestamp() -> String {
    chrono::Local::now().format("[%H:%M:%S]").to_string()
}

fn titleCallback(title: &str) {
    println!("{} {}\n", current_timestamp(), title); 
    notify_rust::Notification::new()
        .summary("Now playing")
        .body(title)
        .icon("waves.gif")
        .show().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No argument");
    }

    //Clear term screen
    let output = std::process::Command::new("clear").output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));

    audiostream::open(&args[1], &titleCallback);
}

