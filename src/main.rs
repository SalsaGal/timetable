use std::collections::HashMap;
use std::fs::{read_to_string, File, remove_file};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::StructOpt;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Timetable {
    classes: Vec<Class>,
    timetable: HashMap<(usize, usize), usize>,
}

impl Timetable {
    fn get_class(&self, day: usize, period: usize) -> Option<&Class> {
        if let Some(index) = self.timetable.get(&(day, period)) {
            self.classes.get(*index)
        } else {
            None
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Class {
    name: String,
    todo: Vec<String>,
}

#[derive(clap::Parser)]
struct Args {
    /// List the full timetable
    #[clap(short, long)]
    timetable: bool,

    /// Add a class
    #[clap(short, long)]
    add_class: Vec<String>,

    /// Use a different configuration path (defaults to ~/.timetable)
    #[clap(short, long)]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut changed = false;

    // Load the file
    let file_path = if let Some(config) = args.config {
        PathBuf::from_str(&config).unwrap()
    } else {
        let mut file_path = home::home_dir().unwrap();
        file_path.push(".timetable");
        file_path
    };
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

    if args.timetable {
        let longest_day_length = timetable.timetable.iter().map(|((_, period), _)| *period).max();

        match longest_day_length {
            None => println!("No timetable made!"),
            Some(0) => println!("All days are empty!"),
            Some(longest_day_length) => {
                for i in 0..timetable.timetable.len() {
                    print!("Day {i}\t");
                }
                println!();
                for period in 0..longest_day_length {
                    let period = period as usize;
                    for day in 0..timetable.timetable.len() {
                        if let Some(class) = timetable.get_class(day, period) {
                            print!("{}", class.name);
                        }
                        print!("\t");
                    }
                    println!()
                }
            },
        }
    }

    // Save the file if needed
    if changed {
        remove_file(&file_path).unwrap();
        let mut file = File::create(&file_path).unwrap();
        let contents = serde_json::to_string_pretty(&timetable).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}
