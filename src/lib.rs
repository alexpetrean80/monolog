pub mod note {
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
                    let k = format!("{:02}:{:02}", hour, minute);

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
}

pub mod db {
    use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

    use crate::note::Note;

    pub struct DB {
        pool: Pool<Sqlite>,
    }

    impl DB {
        pub async fn init(url: &str) -> Result<DB, sqlx::Error> {
            DB::try_create_db(url).await?;

            let pool = SqlitePool::connect(url).await?;

            sqlx::query("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY NOT NULL, text VARCHAR(250) NOT NULL, date TIMESTAMPTZ);").execute(&pool).await?;
            Ok(DB { pool })
        }

        async fn try_create_db(url: &str) -> Result<(), sqlx::Error> {
            let exists = Sqlite::database_exists(url).await.unwrap_or(false);

            if !exists {
                Sqlite::create_database(url).await?;
            }

            Ok(())
        }

        pub async fn create_note(&self, input: &Vec<String>) -> Result<(), sqlx::Error> {
            let note = Note::new(input.join(" "));

            sqlx::query("INSERT INTO notes (text, date) VALUES (?,?)")
                .bind(note.text)
                .bind(note.date)
                .execute(&self.pool)
                .await?;

            Ok(())
        }

        pub async fn get_todays_notes(&self) -> Result<Vec<Note>, sqlx::Error> {
            sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE date >= DATETIME('now', 'start of day') AND date < DATETIME('now', 'start of day', '+1 day');  ",
        )
        .fetch_all(&self.pool)
        .await
        }

        pub async fn get_last_notes(&self, count: &u8) -> Result<Vec<Note>, sqlx::Error> {
            sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY date DESC LIMIT ? ;")
                //.bind(no_days)
                .bind(count)
                .fetch_all(&self.pool)
                .await
        }

        pub async fn get_notes_from(
            &self,
            year: &Option<u16>,
            month: &Option<u8>,
            day: &Option<u8>,
        ) -> Result<Vec<Note>, sqlx::Error> {
            use chrono::{Datelike, Local};
            let now = Local::now();
            let current_year = now.year() as u16;
            let current_month = now.month() as u8;
        
            let mut final_year = *year;
            let mut final_month = *month;
            let final_day = *day;
        
            if final_day.is_some() {
                if final_month.is_none() {
                    final_month = Some(current_month);
                }
                if final_year.is_none() {
                    final_year = Some(current_year);
                }
            }
        
            if final_month.is_some() && final_year.is_none() {
                final_year = Some(current_year);
            }
        
            let query = match (final_year, final_month, final_day) {
                (None, None, None) => "SELECT * FROM notes".to_string(),
        
                (Some(y), None, None) => format!(
                    "SELECT * FROM notes
                     WHERE date >= DATETIME('{:04}-01-01', 'start of day')
                       AND date < DATETIME('{:04}-01-01', 'start of day', '+1 year');",
                    y, y
                ),
        
                (Some(y), Some(m), None) => format!(
                    "SELECT * FROM notes
                     WHERE date >= DATETIME('{:04}-{:02}-01', 'start of day')
                       AND date < DATETIME('{:04}-{:02}-01', 'start of day', '+1 month');",
                    y, m, y, m
                ),
        
                (Some(y), Some(m), Some(d)) => format!(
                    "SELECT * FROM notes
                     WHERE date >= DATETIME('{:04}-{:02}-{:02}', 'start of day')
                       AND date < DATETIME('{:04}-{:02}-{:02}', 'start of day', '+1 day');",
                    y, m, d, y, m, d
                ),
        
                (None, Some(m), d) => {
                    let y = current_year;
                    if let Some(day) = d {
                        format!(
                            "SELECT * FROM notes
                             WHERE date >= DATETIME('{:04}-{:02}-{:02}', 'start of day')
                               AND date < DATETIME('{:04}-{:02}-{:02}', 'start of day', '+1 day');",
                            y, m, day, y, m, day
                        )
                    } else {
                        format!(
                            "SELECT * FROM notes
                             WHERE date >= DATETIME('{:04}-{:02}-01', 'start of day')
                               AND date < DATETIME('{:04}-{:02}-01', 'start of day', '+1 month');",
                            y, m, y, m
                        )
                    }
                }
        
                (Some(y), None, Some(d)) => {
                    let m = current_month;
                    format!(
                        "SELECT * FROM notes
                         WHERE date >= DATETIME('{:04}-{:02}-{:02}', 'start of day')
                           AND date < DATETIME('{:04}-{:02}-{:02}', 'start of day', '+1 day');",
                        y, m, d, y, m, d
                    )
                }
        
                _ => "SELECT * FROM notes".to_string(),
            };
        
            sqlx::query_as::<_, Note>(&query)
                .fetch_all(&self.pool)
                .await
        }
        
    }
}
