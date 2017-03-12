extern crate notify_rust;
extern crate chrono;
extern crate rand;

use std::env;
use std::fs;
use rand::{thread_rng, Rng};

mod audiostream;

fn random_file(dir: &str) -> Option<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut files: Vec<String> = vec![];
    for path in paths {
        files.push(String::from(path.unwrap().path().to_str().unwrap()));
    }

    if files.len() <= 0 {
        return None;
    }

    let idx = thread_rng().gen_range(0, files.len());

    Some(files.get(idx).unwrap().clone())
}

fn current_timestamp() -> String {
    chrono::Local::now().format("[%H:%M:%S]").to_string()
}

fn notify(title: &str, icon: Option<String>) {
    println!("{} {}\n", current_timestamp(), title); 
    let notify = notify_rust::Notification::new()
        .summary("Now playing")
        .body(title)
        .icon(&icon.unwrap_or(String::new()))
        .show();
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No argument");
    }

    let mut uri: Option<String> = None;
    let mut icondir: Option<String> = None;
    let mut icon: Option<String> = None;

    //Parse args
    let mut iter = args.into_iter().peekable();
    while let Some(arg) = iter.next() {

        if arg == "--icondir" {
            icondir = Some(iter.next().unwrap().clone());
        }

        else if arg == "--icon" {
            icon = Some(iter.next().unwrap().clone());
        }

        else if iter.peek() == None {
            uri = Some(arg);
        }
    }

    if uri == None {
        panic!("No uri supplied!");
    }

    //Closure callback
    let callback = |title: &str| {
        let mut iconOpt: Option<String> = None;

        if icon != None {
            iconOpt = Some(icon.iter().next().unwrap().clone());
        }
        else if icondir != None {
            iconOpt = random_file(&icondir.iter().next().unwrap());
        }

        notify(title, iconOpt);
    };

    //Clear term screen
    let output = std::process::Command::new("clear").output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));

    audiostream::open(&uri.unwrap(), &callback);
}

