use dotenvy::dotenv;
use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};
use super::Item;
use urlencoding;
use std::error::Error;
use std::time::Duration;


//single function to run the web scraper
// It takes a search term and returns a vector of found items or an error.
pub async fn run(search_term: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    // --- Browser Setup and Navigation ---
    let options = LaunchOptions::default_builder()
        .headless(false) // visible for debugging
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let encoded_search_term = urlencoding::encode(search_term);
    let url = format!(
        "https://www.chrono24.com/search/index.htm?dosearch=true&query={}&sortorder=0",
        encoded_search_term
    );

    println!("Navigating to URL: {}", url);
    tab.navigate_to(&url)?;

    println!("Waiting for element to appear...");
    tab.wait_for_element_with_custom_timeout(
        "a.js-article-item",
        Duration::from_secs(30),
    )?;

    println!("Element found! Getting page content...");
    let html = tab.get_content()?;

    // --- HTML Parsing ---
    let document = Html::parse_document(&html);

    let item_container_selector = Selector::parse("a.js-article-item").unwrap();
    let title_selector = Selector::parse(".text-bold.text-ellipsis").unwrap();
    let description_selector = Selector::parse(".text-ellipsis.m-b-2").unwrap();
    let price_selector = Selector::parse(".justify-content-between .text-bold").unwrap();

    let mut items: Vec<Item> = Vec::new();

    for element in document.select(&item_container_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .map_or("N/A".to_string(), |el| {
                el.text().collect::<String>().trim().to_string()
            });
        let description = element
            .select(&description_selector)
            .next()
            .map_or("N/A".to_string(), |el| {
                el.text().collect::<String>().trim().to_string()
            });
        let price = element
            .select(&price_selector)
            .next()
            .map_or("N/A".to_string(), |el| {
                el.text().collect::<String>().trim().to_string()
            });
        let item_url = element
            .value()
            .attr("href")
            .map_or("N/A".to_string(), |href| {
                format!("https://www.chrono24.com{}", href)
            });

        let scraped_item = Item {
            name: title,
            description,
            price,
            url: item_url,
        };

        items.push(scraped_item);
    }

    // Return the final vector of found items
    Ok(items)
}


