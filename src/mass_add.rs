use std::io::Write;

use crate::Timetable;

fn get_line() -> String {
    let mut input = "".to_owned();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_owned();
    input
}

pub fn mass_add(timetable: &mut Timetable) {
    println!("[Class name],[Day],[Period]:");

    loop {
        let input = get_line();
        let tokens: Vec<&str> = input.split(',').collect();
        if input.is_empty() {
            break;
        } else if tokens.len() == 2 {
            special(timetable, &tokens);
        } else if tokens.len() == 3 {
            let name = tokens[0];
            let day = tokens[1];
            let period = tokens[2];

            if let Some(class) = timetable.class_index_from_name(name) {
                if let Ok(day) = day.parse() {
                    if let Ok(period) = period.parse() {
                        timetable.add_period(class, day, period);
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
            eprintln!("Incorrect format (use `[class name],[day],[period]`)");
        }
    }
}

fn special(timetable: &mut Timetable, tokens: &[&str]) {
    match tokens[0] {
        "period series" => {
            let tokens: Vec<&str> = tokens[1].split(' ').collect();
            add_period_series(timetable, tokens[0].parse().unwrap(), tokens[1].parse().unwrap()..tokens[2].parse().unwrap());
        }
        _ => eprintln!("Unknown special series element {}", tokens[0]),
    }
}

fn add_period_series<T>(timetable: &mut Timetable, day: usize, series: T) where T: Iterator<Item = usize> {
    println!("Series Start");
    for period in series {
        let index = loop {
            print!("{period}:");
            std::io::stdout().flush().unwrap();
            if let Some(index) = timetable.class_index_from_name(&get_line()) {
                break index;
            } else {
                eprintln!("No class called that!");
            }
        };
        let day = timetable.timetable.entry(day).or_default();
        day.insert(period, index);
    }
    println!("Series Finish");
}
