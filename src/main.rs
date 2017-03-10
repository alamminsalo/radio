extern crate notify_rust;

use std::env;

mod audiostream;

fn titleCallback(title: &str) {
    println!("Title: {}", title); 
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

