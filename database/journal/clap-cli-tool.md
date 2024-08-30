# Clap CLI Tool

As part of my portfolio project, I needed a way to manage my Turso database that I used to store my journal entries. I used it to build the command line interface for my project.

## Goal

I wanted to be able to drop a markdown file into my `journal` directory, and then run a cli command to add it to the database. I also wanted to be able to list all of the entries in the database and modify them.

## Getting Started

First we need to grab the dependencies.

- [clap](https://github.com/clap-rs/clap)
- [libsql](https://crates.io/crates/libsql)
- [serde](https://crates.io/crates/serde)
- [tokio](https://crates.io/crates/tokio)
- [dotenvy](https://crates.io/crates/dotenvy)
- [walkdir](https://crates.io/crates/walkdir)

## Architecture

I needed 5 commands. In order to use this in the cli, and example would be something like this: `cargo run -- init`. We need the `--` to get cargo out of our way in dev, but in production it would be `program init`.

```rust
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
}
```

## Database Connection

Libsql is the libSQL api we need to set up in order to talk with our db.

```rust
dotenvy::dotenv().ok();
let db_url = std::env::var("TURSO_DATABASE_URL")?;
let db_token = std::env::var("TURSO_AUTH_TOKEN")?;
let db = libsql::Builder::new_remote(db_url, db_token)
    .build()
    .await?;

let conn = db.connect()?;
```

## Commands

Now it's time to implement the commands. Let's look at how I was able to add a file into my journals dir and then get that into the db.

```rust
let cli = Cli::parse();

match &cli.command {
    Some(Commands::Add { title, summary }) => {
        let file = std::fs::File::open(format!("database/journal/{}.md", title))?;
        let content = std::io::read_to_string(file)?;
        let title = title.replace("-", " ");

        conn.execute(
            "INSERT INTO journal_entries (title, summary, content) VALUES (?1, ?2, ?3)",
            params![title, summary.clone(), content],
        )
        .await?;
    }
    // ... other commands
}
```

So for a specific example, this is how I added this entry you are ready now.

```bash
cargo run -- add clap-cli-tool 'A brief overview of how I used Clap to build a simple CRUD interface for my Turso db'
```

## Summary

This took me about 2 hours to implement, so very straightforward. Clap is a great tool that I have always wanted to use. I will probably add to it in the future as a little portfolio cli tool for admin tasks.
