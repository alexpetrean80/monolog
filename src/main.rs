use chrono::Timelike;
use clap::{Parser, Subcommand};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use sqlx::{Pool, Sqlite};

pub mod note;
use note::{Note, Print};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(trailing_var_arg = true)]
        input: Vec<String>,
    },
    Last {
        #[arg()]
        no_notes: u8,
    },
    Today {},
}

impl Commands {
    pub async fn exec(&self, pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        match self {
            Commands::Add { input } => Commands::add_note(input, pool).await,
            Commands::Last { no_notes } => Commands::get_last_notes(*no_notes, pool).await,
            Commands::Today {} => Commands::get_today_notes(pool).await,
        }
    }

    async fn add_note(input: &Vec<String>, pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let note = note::Note::new(input.join(" "));

        sqlx::query("INSERT INTO notes (text, date) VALUES (?,?)")
            .bind(note.text)
            .bind(note.date)
            .execute(&pool)
            .await?;
        Ok(())
    }

    async fn get_last_notes(no_notes: u8, pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY date DESC LIMIT ? ;")
            //.bind(no_days)
            .bind(no_notes)
            .fetch_all(&pool)
            .await?
            .into_iter()
            .map(|n| Note {
                text: n.text,
                date: n.date.with_nanosecond(0).unwrap().with_second(0).unwrap(),
            })
            .collect::<Vec<Note>>()
            .print();

        Ok(())
    }

    async fn get_today_notes(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE date >= DATETIME('now', 'start of day') AND date < DATETIME('now', 'start of day', '+1 day');  ",
        )
        .fetch_all(&pool)
        .await?
        .print();

        Ok(())
    }
}

async fn init_db(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url).await?;
    }
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY NOT NULL, text VARCHAR(250) NOT NULL, date TIMESTAMPTZ);").execute(&pool).await?;
    Ok(pool)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let db_url = "sqlite:monolog.db";

    let pool = init_db(&db_url).await.unwrap();

    if let Err(error) = args.cmd.exec(pool).await {
        eprintln!("Error executing: {}", error);
    }

    Ok(())
}
