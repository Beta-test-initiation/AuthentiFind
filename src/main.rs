mod ledger;
mod scraper;
use dotenvy::dotenv;
use headless_chrome::{Browser, LaunchOptions};
use std::env;
use std::error::Error;
use std::time::Duration;
use urlencoding; // <-- Import the 'urlencoding' crate


#[derive(Debug, Clone)]
struct Item {
    name: String,
    description: String,
    price: String,
    url: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().ok();

    //Ledger setup
    let ledger = ledger::Ledger::new().await?;

    let args: Vec<String> = env::args().collect();

    let search_term = args
        .get(1)
        .ok_or("Error: Please provide a search term as a command-line argument.")?;
  
   
    // Calling the scraper
    let scraped_items = scraper::run(&search_term).await?;
    println!("✅ Found {} items on the page.", scraped_items.len());



   //Process and report items
   for item  in scraped_items{
        let hash = ledger::generate_hash(&item.url);
        match ledger.check_provenance(&hash).await? {
            Some(ledger_item) => {
                //case already in ledger
                println!(
                    "[SEEN BEFORE] {} (First seen on: {})",
                    ledger_item.name,
                    ledger_item.first_seen_at.format("%Y-%m-%d")
                );
            }
            None=> {
                println!(
                    "It's a new item slayy {}", item.name
                );
                ledger.register_item(&item).await?;
            }
        }
   }

    println!("--- End of Report ---\n");


    Ok(())
}