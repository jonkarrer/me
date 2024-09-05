use clap::{Parser, Subcommand};
use libsql::params;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String, summary: String },
    List,
    Init,
    Remove { id: u32 },
    Refresh,
    UpdateSummary { id: u32, summary: String },
    UpdateContent { id: u32, file_name: String },
    UpdateTitle { id: u32, title: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    dotenvy::dotenv().ok();
    let db_url = std::env::var("TURSO_DATABASE_URL")?;
    let db_token = std::env::var("TURSO_AUTH_TOKEN")?;
    let db = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await?;

    let conn = db.connect()?;

    match &cli.command {
        Some(Commands::Add { title, summary }) => {
            let file = std::fs::File::open(format!("frontend/assets/journal/{}.md", title))?;
            let content = std::io::read_to_string(file)?;
            let title = title.replace("-", " ");

            conn.execute(
                "INSERT INTO journal_entries (title, summary, content) VALUES (?1, ?2, ?3)",
                params![title, summary.clone(), content],
            )
            .await?;
        }
        Some(Commands::Refresh) => {
            let mut entries = Vec::new();

            for entry in WalkDir::new("frontend/assets/journal")
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_dir() {
                    continue;
                }
                let title = entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("-", " ")
                    .to_owned();
                let content = std::fs::read_to_string(entry.path()).expect("Failed to read file");
                let summary = "Summary pending";
                entries.push((title, summary, content));
            }

            conn.execute("DELETE FROM journal_entries", ()).await?;

            for (title, summary, content) in entries {
                conn.execute(
                    "INSERT INTO journal_entries (title, summary, content) VALUES (?1, ?2, ?3)",
                    params![title, summary, content],
                )
                .await?;
            }
        }
        Some(Commands::List) => {
            let mut rows = conn
                .query("SELECT id, title FROM journal_entries", ())
                .await?;

            while let Some(row) = rows.next().await.expect("Failed to get next row") {
                let id: u32 = row.get(0).expect("Failed to get id");
                let title: String = row.get(1).expect("Failed to get title");

                println!("{}: {}", id, title);
            }
        }
        Some(Commands::Init) => {
            println!("Initializing journal table ...");
            conn.execute("CREATE TABLE IF NOT EXISTS journal_entries (id INTEGER PRIMARY KEY, title TEXT, summary TEXT, content TEXT)", ()).await.expect("Failed to create table");
            println!("Done");
        }
        Some(Commands::Remove { id }) => {
            conn.execute("DELETE FROM journal_entries WHERE id = ?1", params![id])
                .await?;
        }
        Some(Commands::UpdateSummary { id, summary }) => {
            conn.execute(
                "UPDATE journal_entries SET summary = ?2 WHERE id = ?1",
                params![id, summary.clone()],
            )
            .await?;
        }
        Some(Commands::UpdateContent { id, file_name }) => {
            let file = std::fs::File::open(format!("frontend/assets/journal/{}.md", file_name))?;
            let content = std::io::read_to_string(file)?;
            conn.execute(
                "UPDATE journal_entries SET content = ?2 WHERE id = ?1",
                params![id, content.clone()],
            )
            .await?;
        }
        Some(Commands::UpdateTitle { id, title }) => {
            conn.execute(
                "UPDATE journal_entries SET title = ?2 WHERE id = ?1",
                params![id, title.clone()],
            )
            .await?;
        }
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
