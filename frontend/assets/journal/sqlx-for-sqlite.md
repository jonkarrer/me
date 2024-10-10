# Using Sqlx for SQLite

Recently had a project that needed to utilize all the benefits of SQLite, and Rust was interacting with the database. I have experience with `sqlx` already, so naturally I chose this to get going. Sqlx is a fantastic library that just get's the job done and it supports async features.

## Installation

The crate can be found here: [Sqlx on crates.io](https://crates.io/crates/sqlx)

```bash
cargo add sqlx --features sqlite
```

Some other features I used as well:

```bash
cargo add sqlx -F runtime-tokio-rustls,macros,chrono
```

## Migrations

`sqlx` supports migrations, and it's pretty straight forward. I have a `migrations` folder where they are stored. This is the command to run for adding a new migration. The source defaults to `./migrations`, but you can override that like I did here.

```bash
sqlx migrate add <migration_name> --source database/migrations 
```

Inside this file I added a create table statement, like so:

```sql
CREATE TABLE IF NOT EXISTS monthly_stock_bars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_datetime TEXT NOT NULL,
    event_unix_timestamp INTEGER NOT NULL,
    open_price REAL NOT NULL DEFAULT 0.0,
    close_price REAL NOT NULL DEFAULT 0.0,
    high_price REAL NOT NULL DEFAULT 0.0,
    low_price REAL NOT NULL DEFAULT 0.0,
    volume REAL NOT NULL DEFAULT 0.0,
    volume_weighted_price REAL DEFAULT 0.0,
    stock_symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    bar_trend TEXT NOT NULL,
    buy_or_sell INTEGER NOT NULL,
    next_frame_price REAL NOT NULL,
    next_frame_trend TEXT NOT NULL,
    next_frame_unix_timestamp INTEGER NOT NULL,
    next_frame_event_datetime TEXT NOT NULL,
    ten_week_moving_avg REAL NOT NULL,
    ten_week_rsi REAL NOT NULL,
    ten_week_ema REAL NOT NULL,
    ten_week_high REAL NOT NULL,
    ten_week_low REAL NOT NULL,
    five_week_high REAL NOT NULL,
    five_week_low REAL NOT NULL
);
```

And now to run this migration, we use this command. The important note here is that `sqlx` need the database URL in the environment variable `DATABASE_URL` to work.

```bash
export DATABASE_URL=sqlite:///Volumes/stocks.db && sqlx migrate run --source database/migrations
```

## Database Connection

I prefer a singleton pattern when dealing with database connections. It keeps the code for connecting and testing the database all in one file. Here is how to connect to a SQLite database with `sqlx`. Also, there are some utility functions for testing and resetting the database.

```rust
use sqlx::{migrate::MigrateDatabase, SqlitePool};

pub struct SqliteDb {
    pub uri: String,
    pub pool: SqlitePool,
}

impl SqliteDb {
    pub async fn connect(uri: &str) -> Result<Self> {
        println!("Connecting to {}...", uri);
        let pool = SqlitePool::connect(uri).await?;
        println!("Connected!");
        Ok(Self {
            uri: uri.to_string(),
            pool,
        })
    }

    pub async fn create_new(uri: &str) -> Result<Self> {
        let does_exist = sqlx::Sqlite::database_exists(uri).await.unwrap_or(false);

        if does_exist {
            println!("Database already exists, connecting...");
            let connection = Self::connect(uri).await?;
            return Ok(connection);
        }

        println!("Creating database at {}", uri);
        sqlx::Sqlite::create_database(uri).await?;

        println!("Database created, connecting...");
        let connection = Self::connect(uri).await?;
        Ok(connection)
    }

    pub async fn test_connection(&self) {
        match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => println!("Connection successful!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub async fn reset_database(uri: &str) -> Result<()> {
        println!("Resetting database at {}...", uri);
        sqlx::Sqlite::drop_database(uri).await?;
        println!("Database reset!");
        Self::create_new(uri).await?;
        Ok(())
    }
}
```

Boom, ready to rock.

## Using Database

My favorite way to build structure querying my database connection is by leveraging Rust's traits. Traits are powerful ways to decouple the logic for each data model's repository. In complex projects, it can be a huge benefit, and you can pass the traits around as interfaces instead of the concrete database connection. Here is an example:

```rust
pub trait MonthlyStockBarRepository {
    async fn insert_monthly_stock_bar(&self, model_entry: &MonthlyStockBarModelEntry)
        -> Result<()>;
    async fn insert_batch_of_monthly_stock_bars(
        &self,
        model_entries: &[MonthlyStockBarModelEntry],
    ) -> Result<()>;
}

impl MonthlyStockBarRepository for SqliteDb {
    async fn insert_monthly_stock_bar(
        &self,
        model_entry: &MonthlyStockBarModelEntry,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO monthly_stock_bars (event_datetime, event_unix_timestamp, open_price, close_price, high_price, low_price, volume, volume_weighted_price, stock_symbol, timeframe, bar_trend, buy_or_sell, next_frame_price, next_frame_trend, next_frame_unix_timestamp, next_frame_event_datetime, ten_week_moving_avg, ten_week_ema, ten_week_rsi, ten_week_high, ten_week_low, five_week_high, five_week_low)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&model_entry.event_datetime)
        .bind(&model_entry.event_unix_timestamp)
        .bind(&model_entry.open_price)
        .bind(&model_entry.close_price)
        .bind(&model_entry.high_price)
        .bind(&model_entry.low_price)
        .bind(&model_entry.volume)
        .bind(&model_entry.volume_weighted_price)
        .bind(&model_entry.stock_symbol)
        .bind(&model_entry.timeframe)
        .bind(&model_entry.bar_trend)
        .bind(&model_entry.buy_or_sell)
        .bind(&model_entry.next_frame_price)
        .bind(&model_entry.next_frame_trend)
        .bind(&model_entry.next_frame_unix_timestamp)
        .bind(&model_entry.next_frame_event_datetime)
        .bind(&model_entry.ten_week_sma)
        .bind(&model_entry.ten_week_ema)
        .bind(&model_entry.ten_week_rsi)
        .bind(&model_entry.ten_week_high)
        .bind(&model_entry.ten_week_low)
        .bind(&model_entry.five_week_high)
        .bind(&model_entry.five_week_low)
        .execute(&self.pool).await?;

        Ok(())
    }

    // Use a transaction as to insert all at once instead of one at a time
    async fn insert_batch_of_monthly_stock_bars(
        &self,
        model_entries: &[MonthlyStockBarModelEntry],
    ) -> Result<()> {
        let transaction = self.pool.begin().await?;
        for model in model_entries {
            self.insert_monthly_stock_bar(model).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
```

And when I need to use the Database with this trait, I just call the functions like so:

```rust
// Bring in the trait
use database::{MonthlyStockBarModelEntry, MonthlyStockBarRepository};

let db = SqliteDb::connect("sqlite:///Volumes/stocks.db").await?;

db.insert_monthly_stock_bar(&model_entry).await?;

for model_entry in model_entries {
    db.insert_batch_of_monthly_stock_bars(&model_entries).await?;
}
```

## Conclusion

Nothing too crazy here. This is a simple rundown of how to use the `sqlx` library for SQLite.
