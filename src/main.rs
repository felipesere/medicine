use anyhow::Context;
use home::home_dir;
use jiff::Zoned;
use jiff::civil::{DateTime, Time};
use jiff_sqlx::ToSqlx;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Executor, Sqlite, migrate::Migrator};

use clap::{Args, Parser, Subcommand};

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[derive(Parser)]
#[command(name = "medicine")]
#[command(about = "A CLI application to track medicine usage")]
#[command(version = "1.0")]
#[command(propagate_version = true)]
#[derive(Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a medicine entry
    Add(AddArgs),
    // Other commands could be added here, like `list`, `remove`, etc.
}

#[derive(Args, Debug)]
struct AddArgs {
    /// Name of the medicine
    name: String,

    /// Dosage of the medicine (e.g., 1, 2, or 3)
    dosage: Option<usize>,

    /// The time when the medicine should be taken
    #[arg(long, short)]
    at: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let home = home_dir().context("Could not find users home")?;
    let sqlite_db = home.join(".medicine.sqlite").display().to_string();

    if !Sqlite::database_exists(&sqlite_db).await.unwrap_or(false) {
        println!("Creating database {}", &sqlite_db);
        match Sqlite::create_database(&sqlite_db).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }
    let pool = SqlitePoolOptions::new().connect(&sqlite_db).await?;

    MIGRATOR.run(&pool).await?;

    match cli.command {
        Commands::Add(args) => {
            let medicine = args.name;
            let dosage = args.dosage.unwrap_or(1) as i32;
            let time_taken = if let Some(time) = args.at {
                parse_time_to_today(&time)?
            } else {
                DateTime::from(Zoned::now())
            };

            let outcome =
                sqlx::query("INSERT INTO medicines (name, dosage, time_taken) VALUES ($1, $2, $3)")
                    .bind(medicine)
                    .bind(dosage)
                    .bind(time_taken.to_sqlx())
                    .execute(&pool)
                    .await?;
            if outcome.rows_affected() != 1 {
                println!("Failed to insert medicine");
            }
        }
    }

    Ok(())
}

/// Parse a time string (HH:MM) into a DateTime for today at that time
fn parse_time_to_today(time_str: &str) -> Result<DateTime, anyhow::Error> {
    let time_taken: Time = time_str.parse()?;
    let now = Zoned::now();
    let outcome = now.with().time(time_taken).build()?;

    Ok(outcome.into())
}
