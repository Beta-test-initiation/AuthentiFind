use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    price: String,
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Correctly build the launch options, handling the Result with '?'
    let options = LaunchOptions::default_builder()
        .headless(false)
        .build()?; // 

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let url = "https://www.chrono24.com/search/index.htm?dosearch=true&watchTypes=U&searchexplain=false&query=Vintage+Rolex&sortorder=0";
    println!("Navigating to URL: {}", url);

    tab.navigate_to(url)?;

    println!("Waiting for element to appear...");

    // Wait for the element with a longer, custom timeout
    tab.wait_for_element_with_custom_timeout(
        "a.js-article-item",
        Duration::from_secs(30), // Wait for up to 30 seconds
    )?;

    println!("Element found! Getting page content...");
    let html = tab.get_content()?;

    let document = Html::parse_document(&html);

    // selectors and parsing logic 
    let item_container_selector = Selector::parse("a.js-article-item").unwrap();
    let title_selector = Selector::parse(".text-bold.text-ellipsis").unwrap();
    let description_selector = Selector::parse(".text-ellipsis.m-b-2").unwrap();
    let price_selector = Selector::parse(".d-flex .text-bold").unwrap();
    
    let mut items: Vec<Item> = Vec::new();

    for element in document.select(&item_container_selector) {
        let title = element.select(&title_selector).next().map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
        let description = element.select(&description_selector).next().map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
        let price = element.select(&price_selector).next().map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
        let item_url = element.value().attr("href").map_or("N/A".to_string(), |href| format!("https://www.chrono24.com{}", href));
        
        items.push(Item { name: title, description, price, url: item_url });
    }

    if items.is_empty() {
        println!("⚠️ Found 0 items even after waiting. The page may be blocked.");
    } else {
        println!("✅ Found {} items.", items.len());
        println!("{:#?}", items);
    }

    Ok(())
}