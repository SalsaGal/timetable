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
    let args = Args::parse();

    let mut file_path = home::home_dir().unwrap();
    file_path.push(".timetable");

    let file_text = read_to_string(&file_path).unwrap_or_else(|_| {
        let mut file = File::create(&file_path).unwrap();
        let contents = serde_json::to_string_pretty(&Timetable::default()).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        read_to_string(&file_path).unwrap()
    });
    let timetable: Timetable = serde_json::from_str(&file_text).unwrap();
}
