use crate::Timetable;

pub fn mass_add(timetable: &mut Timetable) {
    println!("[Class name],[Day],[Period]:");

    loop {
        let mut input = "".to_owned();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_owned();
        let tokens: Vec<&str> = input.split(',').collect();
        if input.is_empty() {
            break;
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
