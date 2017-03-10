extern crate notify_rust;
extern crate chrono;

use std::env;

mod audiostream;

fn current_timestamp() -> String {
    chrono::Local::now().format("[%H:%M:%S]").to_string()
}

fn titleCallback(title: &str) {
    println!("{} {}", current_timestamp(), title); 
    notify_rust::Notification::new()
        .summary("Now playing")
        .body(title)
        .show().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No argument");
    }

    audiostream::open(&args[1], &titleCallback);
}

