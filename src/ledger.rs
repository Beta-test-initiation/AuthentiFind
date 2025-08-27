// in src/ledger.rs

use sha2::{Digest, Sha256};
use sqlx::{FromRow, SqlitePool};
use std::env;
use chrono::NaiveDateTime; 



#[derive(Debug, FromRow)]
pub struct LedgerItem {
    pub id: Option<i64>,
    pub name: String,
    pub url: String,
    pub item_hash: String,
    pub first_seen_at: NaiveDateTime, 
}

// This function can now be used by other modules
pub fn generate_hash(url: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(url.as_bytes());
    let hash_bytes = hasher.finalize();
    format!("{:x}", hash_bytes)
}


// This struct will hold our database connection pool.
pub struct Ledger {
    pool: SqlitePool,
}

impl Ledger {
    // Connects to the database and returns a new Ledger instance.
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(Ledger { pool })
    }

    // Checks if an item exists based on its unique hash.
    pub async fn check_provenance(&self, item_hash: &str) -> Result<Option<LedgerItem>, sqlx::Error> {
        // The sqlx::query_as! macro checks your SQL and types at compile time!
        let item = sqlx::query_as!(
            LedgerItem,
            "SELECT id, name, url, item_hash, first_seen_at FROM items WHERE item_hash = ?",
            item_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(item)
    }

    // Registers a new item, but only if it doesn't already exist.
    pub async fn register_item(&self, item: &super::Item) -> Result<LedgerItem, sqlx::Error> {
        // Create a unique hash for the item based on its URL.
        let mut hasher = Sha256::new();
        hasher.update(item.url.as_bytes());
        let hash_bytes = hasher.finalize();
        let item_hash = generate_hash(&item.url);

        // First, check if an item with this hash already exists.
        if let Some(existing_item) = self.check_provenance(&item_hash).await? {
            println!("Item already in ledger: {}", existing_item.name);
            return Ok(existing_item);
        }

        // If not found, insert the new item into the database.
        println!("New item found! Registering: {}", item.name);
        let new_item = sqlx::query_as!(
            LedgerItem,
            r#"
            INSERT INTO items (name, description, price, url, item_hash)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id, name, url, item_hash, first_seen_at
            "#,
            item.name,
            item.description,
            item.price,
            item.url,
            item_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(new_item)
    }
}