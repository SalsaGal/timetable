mod mass_add;

use std::collections::HashMap;
use std::fs::{read_to_string, File, remove_file};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::StructOpt;
use cli_table::{Cell, Table, print_stdout, Style};
use mass_add::mass_add;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Timetable {
    pub classes: Vec<Class>,
    pub timetable: HashMap<usize, HashMap<usize, usize>>,
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
pub struct Class {
    name: String,
    todo: Vec<String>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self {
            name,
            todo: vec![],
        }
    }
}

/// Run without arguments to simply print the timetable
#[derive(clap::Parser)]
struct Args {
    /// List the full timetable
    #[clap(short, long)]
    timetable: bool,

    /// Add a class
    #[clap(long)]
    add_class: Vec<String>,

    /// Add a period, uses the format `-a [class name],[day],[period]`
    #[clap(short, long)]
    add_period: Vec<String>,

    /// Remove a period from the timetable, uses the format `-r [day],[period]`
    #[clap(short, long)]
    rm_period: Vec<String>,

    /// Use a different configuration path (defaults to ~/.timetable)
    #[clap(short, long)]
    config: Option<String>,

    /// Enter another menu to add several periods without typing in the full command each time.
    #[clap(short, long)]
    mass_add_period: bool,
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
        if timetable.classes.iter().any(|c| c.name == class) {
            eprintln!("Already has class called {class}");
        } else {
            timetable.classes.push(Class {
                name: class,
                todo: vec![],
            });
            changed = true;
        }
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
            eprintln!("-a takes the format of `-a [class name],[day],[period]`");
        }
    }

    for period in args.rm_period {
        let tokens: Vec<&str> = period.split(',').collect();
        if tokens.len() == 2{
            if let Ok(day) = tokens[0].parse() {
                if let Ok(period) = tokens[1].parse() {
                    if let Some(day_map) = timetable.timetable.get_mut(&day) {
                        if day_map.remove(&period).is_some() {
                        } else {
                            eprintln!("No period {period} on day {day}");
                            changed = true;
                        }
                    } else {
                        eprintln!("No day {day}");
                    }
                } else {
                    eprintln!("Day is in invalid format (must be a number)!");
                }
            } else {
                eprintln!("Day is in invalid format (must be a number)!");
            }
        } else {
            eprintln!("-r takes the format of `-r [day],[period]`");
        }
    }

    if args.mass_add_period {
        mass_add(&mut timetable);
        changed = true;
    }

    if args.timetable || !changed {
        // I think i did this a bit stupidly
        let day_bounds = {
            if timetable.timetable.is_empty() {
                None
            } else {
                let mut max = 0;
                let mut min = usize::MAX;
                for day in timetable.timetable.values() {
                    if !day.is_empty() {
                        max = usize::max(max, *day.keys().max().unwrap_or(&0));
                        min = usize::min(min, *day.keys().min().unwrap_or(&0));
                    }
                }
                Some((max, min))
            }
        };
        let day_max = timetable.timetable.keys().max();
        let day_min = timetable.timetable.keys().min();

        if let Some((latest_period, earliest_period)) = day_bounds {
            if let Some(day_max) = day_max {
                let day_min = day_min.unwrap();
                let mut table = vec![];
                let mut title = vec!["".cell()];
                for day in *day_min..*day_max + 1 {
                    title.push(format!("Day {day}").cell().bold(true));
                }
                for period in earliest_period..latest_period + 1 {
                    table.push(vec![format!("Period {period}").cell().bold(true)]);
                    for day in *day_min..*day_max + 1 {
                        let cell = period - earliest_period;
                        if let Some(class) = timetable.get_class(day, period) {
                            table[cell].push((&class.name).cell());
                        } else {
                            table[cell].push("".cell());
                        }
                    }
                }
    
                print_stdout(table.table().title(title)).unwrap();
            } else {
                eprintln!("No days in timetable!");
            }
        } else {
            println!("No timetable made!")
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
