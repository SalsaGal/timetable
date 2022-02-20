use std::fs::{read_to_string, File, remove_file};
use std::io::Write;

use clap::StructOpt;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Timetable {
    classes: Vec<Class>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Class {
    name: String,
    todo: Vec<String>,
}

#[derive(clap::Parser)]
struct Args {
    /// List the complete timetable
    #[clap(short, long)]
    list: bool,

    /// Add a class
    #[clap(short, long)]
    add_class: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut changed = false;

    // Load the file
    let mut file_path = home::home_dir().unwrap();
    file_path.push(".timetable");
    let mut timetable = if let Ok(text) = read_to_string(&file_path) {
        serde_json::from_str(&text).unwrap()
    } else {
        File::create(&file_path).unwrap();
        changed = true;
        Timetable::default()
    };

    for class in args.add_class {
        timetable.classes.push(Class {
            name: class,
            todo: vec![],
        });
        changed = true;
    }

    // Save the file if needed
    if changed {
        remove_file(&file_path).unwrap();
        let mut file = File::create(&file_path).unwrap();
        let contents = serde_json::to_string_pretty(&timetable).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}
