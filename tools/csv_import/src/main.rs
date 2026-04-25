/// CSV import tool.
/// Usage: csv_import --file data.csv --table tablename [--schema app_myproject]
///
/// The Ops agent generates the actual INSERT logic per-project based on the CSV headers
/// and the target table schema. This file is the harness; the agent fills in the body.
use clap::Parser;
use sqlx::PgPool;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Import a CSV file into a Supabase Postgres table")]
struct Args {
    #[arg(long)]
    file: PathBuf,
    #[arg(long)]
    table: String,
    #[arg(long, default_value = "public")]
    schema: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let args = Args::parse();
    let db_url = std::env::var("SUPABASE_DB_URL").expect("SUPABASE_DB_URL must be set");

    let pool = PgPool::connect(&db_url).await?;
    sqlx::query(&format!("SET search_path TO {}", args.schema))
        .execute(&pool)
        .await?;

    let mut reader = csv::Reader::from_path(&args.file)?;
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();

    let mut count = 0usize;
    for result in reader.records() {
        let record = result?;
        let values: Vec<&str> = record.iter().collect();

        // Build parameterised INSERT dynamically from headers.
        // The Ops agent replaces this generic implementation with a typed one
        // once it knows the target table schema.
        let cols = headers.join(", ");
        let placeholders: Vec<String> = (1..=headers.len()).map(|i| format!("${}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT DO NOTHING",
            args.table,
            cols,
            placeholders.join(", ")
        );

        let mut q = sqlx::query(&sql);
        for v in &values {
            q = q.bind(*v);
        }
        q.execute(&pool).await?;
        count += 1;
    }

    println!("Imported {} rows into {}.{}", count, args.schema, args.table);
    Ok(())
}
