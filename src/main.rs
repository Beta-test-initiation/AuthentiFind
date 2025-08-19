use reqwest;
use scraper::{Html, Selector};
use std::fs;

#[derive(Debug)] //Print struct for debugging
struct Item {
    name: String,
    description:String,
    price: String,
    url: String,
} 


#[tokio::main] //macro that transforms async fn main into a standard fn main that initializes the tokio runtime.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AuthentiFind Scraper Initialising ..");

    

    //Scraper logic
    let url = "https://www.chrono24.com/search/index.htm?dosearch=true&watchTypes=U&searchexplain=false&query=Vintage+Rolex&watchCategories=&caseMaterials=&countryIds=&gender=&braceletMaterial=&maxAgeInDays=&priceFrom=&priceTo=&sortorder=0&STARTSEARCH=&SEARCHSTATS_BRAND_ID=&SEARCHSTATS_MATERIAL_ID=&_sourcePage=rWSP8lSk_G_1aChwEDxU090i6hCnSHazOAY8AbZk2diFM9tnfYCTVd2EnwqgBOeQLdCzSVnbcgqtHQJr-m8UeSO1bIS7w-EVmrpm6omRDxI%3D&__fp=Usz26zsYYzWgk40GSaqmfBPRyfAgVU2zvS2nGUzrKWAQEalFR1f8MqiVD96FgsBmxCU7SKsaInmhtUneUJLwkPVx8lhL_oZnxHMg7GbpZAh_KfQGbIe1iw40a4vMqGgk3eeWHRhNe8eJ3L6cNOYooN7oZ-0xs0QAhyohsGlEPCGBS4IYxXjFrDy2gZrpV_FsQsMNOJFMEkzGO64woBKA0D1nVBCAQjgv-j572TntUxwVJc2gWwhSd0nmgw0vkH9Y&__at=1755646160.5qtctiCrl_l5G9CYFGmcXJPtP6RN6cBtUQlr91OmGww.AXG1VdiTCWX8hb01_73_YtXKwnIn";

    println!("Fetching URL: {}", url);

    // To appear more like a browser, we add a User-Agent header.
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?;
        
    let body = response.text().await?;

    // --- DEBUG STEP: SAVE THE HTML TO A FILE ---
    fs::write("output.html", &body)?;
    println!("✅ HTML content saved to output.html");
    // --- END DEBUG STEP ---


    let document = Html::parse_document(&body);


    //Define selectors 
    let item_container_selector = Selector::parse("a.js-article-item").unwrap();
    let title_selector = Selector::parse(".text-bold.text-ellipsis").unwrap();
    let description_selector = Selector::parse(".text-ellipsis.m-b-2").unwrap();
    let price_selector = Selector::parse(".d-flex .text-bold").unwrap(); // Target the <strong> tag inside the price div


    let mut items: Vec<Item> = Vec::new(); //mutable vector of items

    for element in document.select(&item_container_selector) {
        let title = element 
                .select(&title_selector)
                .next()
                .map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
    
        let description = element
            .select(&description_selector)
            .next()
            .map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
    
        let price = element
            .select(&price_selector)
            .next()
            .map_or("N/A".to_string(), |el| el.text().collect::<String>().trim().to_string());
        
        // Extract the URL from the container's 'href' attribute
        let item_url = element
            .value()
            .attr("href")
            .map_or("N/A".to_string(), |href| format!("https://www.chrono24.com{}", href));
        
        items.push(Item {
            name: title,
            description,
            price,
            url: item_url,
        });
    
    }

    println!("Found {} items.", items.len());
    println!("{:#?}", items);

    Ok(())

}
