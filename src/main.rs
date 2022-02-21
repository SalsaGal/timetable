use std::collections::HashMap;
use std::fs::{read_to_string, File, remove_file};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::StructOpt;
use cli_table::{Cell, Table, print_stdout, Style};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Timetable {
    classes: Vec<Class>,
    timetable: HashMap<usize, HashMap<usize, usize>>,
}

impl Timetable {
    fn class_index_from_name(&self, name: &str) -> Option<usize> {
        let class = self.classes.iter().enumerate().find(|(_, class)| class.name == name);
        if let Some((index, _)) = class {
            Some(index)
        } else {
            None
        }
    }

    fn get_class(&self, day: usize, period: usize) -> Option<&Class> {
        if let Some(day) = self.timetable.get(&day) {
            if let Some(index) = day.get(&period) {
                self.classes.get(*index)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_period(&mut self, class: usize, day: usize, period: usize) {
        let day = self.timetable.entry(day).or_default();
        day.insert(period, class);
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
    #[clap(long)]
    add_class: Vec<String>,

    /// Add a period, uses the format `--add-period [class name],[day],[period]`
    #[clap(long)]
    add_period: Vec<String>,

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
        // TODO dont allow duplicate class names
        timetable.classes.push(Class {
            name: class,
            todo: vec![],
        });
        changed = true;
    }

    for period in args.add_period {
        let tokens: Vec<&str> = period.split(',').collect();
        if tokens.len() == 3 {
            let name = tokens[0];
            let day = tokens[1];
            let period = tokens[2];

            if let Some(class) = timetable.class_index_from_name(name) {
                if let Ok(day) = day.parse() {
                    if let Ok(period) = period.parse() {
                        timetable.add_period(class, day, period);
                        changed = true;
                    } else {
                        eprintln!("Day is in invalid format (must be a number)!");
                    }
                } else {
                    eprintln!("Day is in invalid format (must be a number)!");
                }
            } else {
                eprintln!("No class called {name}!");
            }
        } else {
            eprintln!("--add-period takes the format of `--add-period [class name],[day],[period]`");
        }
    }

    if args.timetable {
        let longest_day_length = {
            if timetable.timetable.is_empty() {
                None
            } else {
                let mut max = 0;
                for day in timetable.timetable.values() {
                    max = usize::max(max, day.len());
                }
                Some(max)
            }
        };
        let day_count = *timetable.timetable.keys().max().unwrap_or(&0) + 1;

        match longest_day_length {
            None => println!("No timetable made!"),
            Some(longest_day_length) => {
                let mut table = vec![];
                let mut title = vec!["".cell()];
                for day in 0..day_count {
                    title.push(format!("Day {day}").cell().bold(true));
                }
                for period in 0..longest_day_length {
                    table.push(vec![format!("Period {period}").cell().bold(true)]);
                    for day in 0..day_count {
                        if let Some(class) = timetable.get_class(day, period) {
                            table[period].push((&class.name).cell());
                        } else {
                            table[period].push("".cell());
                        }
                    }
                }

                print_stdout(table.table().title(title)).unwrap();
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
