use clap::{Parser, Subcommand};
use monolog::{db::DB, note::Print};

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
    pub async fn exec(&self, db: &DB) -> Result<(), sqlx::Error> {
        match self {
            Commands::Add { input } => db.create_note(input).await,
            Commands::Last { no_notes } => {
                let notes = db.get_last_notes(no_notes).await?;
                notes.print();
                Ok(())
            }
            Commands::Today {} => {
                let notes = db.get_todays_notes().await?;
                notes.print();
                Ok(())
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let db_url = "sqlite:monolog.db";

    let db = DB::init(&db_url).await.unwrap();

    if let Err(error) = args.cmd.exec(&db).await {
        eprintln!("Error executing: {}", error);
    }

    Ok(())
}
