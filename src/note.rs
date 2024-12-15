use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveDate, Timelike};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Note {
    pub text: String,
    pub date: DateTime<Local>,
}

impl Note {
    pub fn new(text: String) -> Note {
        Note {
            text,
            date: Local::now(),
        }
    }
}

pub trait Print {
    fn print(self);
}

impl Print for Vec<Note> {
    fn print(self) {
        let mut per_days: HashMap<NaiveDate, Vec<Note>> = HashMap::new();

        for n in self {
            let day = n.date.date_naive();

            per_days.entry(day).or_insert_with(Vec::new).push(n);
        }

        let mut days: Vec<&NaiveDate> = per_days.keys().collect();
        days.sort();

        for d in days {
            println!("# {}\n", d);

            let notes_per_day = per_days.get(d).unwrap();
            let mut per_time: HashMap<String, Vec<String>> = HashMap::new();

            for n in notes_per_day {
                let hour = n.date.time().hour();
                let minute = n.date.time().minute();
                let k = format!("{}:{}", hour, minute);

                per_time
                    .entry(k)
                    .or_insert_with(Vec::new)
                    .push(n.text.clone());
            }

            let mut times: Vec<&String> = per_time.keys().collect();
            times.sort();

            for time in times {
                println!("## {}\n", time);
                let texts = per_time.get(time).unwrap();
                for t in texts {
                    println!("> {}", t);
                }

                println!("");
            }
        }
    }
}
