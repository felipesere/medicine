use std::collections::HashMap;
use std::fmt::Display;

use anyhow::Context;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use home::home_dir;
use jiff::civil::{DateTime, Time};
use jiff::tz::TimeZone;
use jiff::{Unit, Zoned};
use jiff_sqlx::ToSqlx;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{Sqlite, migrate::Migrator};

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
    List(ListArgs),
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

#[derive(Args, Debug)]
struct ListArgs {
    #[clap(default_value_t = ListMode::TwentyFourHours)]
    mode: ListMode,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
enum ListMode {
    All,
    #[clap(name = "24h")]
    #[default]
    TwentyFourHours,
}

impl Display for ListMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListMode::All => f.write_str("all"),
            ListMode::TwentyFourHours => f.write_str("24h"),
        }
    }
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
    let pool: SqlitePool = SqlitePoolOptions::new().connect(&sqlite_db).await?;

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
        Commands::List(args) => {
            let query = match args.mode {
                ListMode::All => {
                    r#"SELECT name, dosage, time_taken FROM medicines ORDER BY datetime(time_taken)"#
                }
                ListMode::TwentyFourHours => {
                    r#"SELECT name, dosage, time_taken FROM medicines WHERE "time_taken" >= date('now', '-1 days') ORDER BY datetime(time_taken)"#
                }
            };

            let medicines: Vec<(String, i32, jiff_sqlx::DateTime)> =
                sqlx::query_as(query).fetch_all(&pool).await?;
            let mut meds_table = Table::new();
            meds_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(80)
                .set_header(vec!["Name", "Dosage", "Time Taken", "Since"]);

            let now = DateTime::from(Zoned::now()).round(Unit::Minute)?;
            let mut running_totals: HashMap<String, i32> = HashMap::new();
            for (name, dosage, time_taken) in medicines {
                running_totals
                    .entry(name.clone())
                    .and_modify(|c| *c += dosage)
                    .or_insert(dosage);

                let time_taken = time_taken.to_jiff().round(Unit::Minute)?;
                let elapsed_time = time_taken.since(now).unwrap();
                let formtttable = time_taken.to_zoned(TimeZone::UTC)?;
                meds_table.add_row(vec![
                    name.to_string(),
                    format!("{dosage}x"),
                    formtttable.strftime("%F at %H:%M").to_string(),
                    format!("{:#}", elapsed_time),
                ]);
            }

            println!("{meds_table}");

            let mut totals_table = Table::new();
            totals_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(80)
                .set_header(vec!["Name", "Total Dosage"]);

            for (name, total_dosage) in running_totals {
                totals_table.add_row(vec![name, format!("{total_dosage}")]);
            }

            println!("{totals_table}")
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
