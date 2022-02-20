use std::fs::{read_to_string, File};
use std::io::Write;

use clap::StructOpt;

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Timetable {
    classes: Vec<Class>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Class {
    name: String,
    todo: Vec<String>,
}

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    list: bool,
}

fn main() {
    let mut file_path = home::home_dir().unwrap();
    file_path.push(".timetable");

    if let Ok(text) = read_to_string(&file_path) {
        // Try to load timetable info
    } else {
        // Create timetable info
        let mut file = File::create(&file_path).unwrap();
        let contents = serde_json::to_string_pretty(&Timetable::default()).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    let args = Args::parse();
}
