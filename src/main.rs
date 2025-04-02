use reqwest::Error;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashSet;
use std::io::{self, Write};
use url::Url;
use std::pin::Pin;

#[derive(Serialize, Debug)]
struct DocumentationData {
    title: String,
    url: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_url = get_user_input("Enter the starting documentation URL to scrape: ");
    let max_depth: usize = get_user_input("Enter the maximum recursion depth: ")
        .parse()
        .unwrap_or(2); // Default depth is 2 if input is invalid

    println!("Starting recursive scraping from: {}", start_url);

    let mut visited_urls = HashSet::new();
    let mut all_scraped_data = Vec::new();

    // Perform recursive scraping
    recursive_scrape(
        &start_url,
        0,
        max_depth,
        &mut visited_urls,
        &mut all_scraped_data,
    )
        .await?;

    // Save all scraped data to a JSON file
    save_to_file(&all_scraped_data, "all_scraped_data.json").unwrap();

    println!("Scraping completed. Data saved to all_scraped_data.json");
    Ok(())
}

// Pin the recursive future to use Box effectively
fn recursive_scrape<'a>(
    url: &'a str,
    current_depth: usize,
    max_depth: usize,
    visited_urls: &'a mut HashSet<String>,
    all_scraped_data: &'a mut Vec<DocumentationData>,
) -> Pin<Box<dyn std::future::Future<Output = Result<(), Error>> + 'a>> {
    Box::pin(async move {
        if current_depth >= max_depth {
            return Ok(());
        }

        if visited_urls.contains(url) {
            return Ok(());
        }

        println!("Scraping URL (depth={}): {}", current_depth, url);
        visited_urls.insert(url.to_string());

        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            eprintln!("Failed to fetch URL: {}. Status: {}", url, response.status());
            return Ok(());
        }

        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let page_data = scrape_page_data(&document, url);
        all_scraped_data.push(page_data);

        let link_selector = Selector::parse("a").unwrap();
        let links: Vec<String> = document
            .select(&link_selector)
            .filter_map(|element| element.value().attr("href"))
            .filter_map(|href| make_absolute_url(url, href))
            .collect();

        for link in links {
            recursive_scrape(&link, current_depth + 1, max_depth, visited_urls, all_scraped_data)
                .await?;
        }

        Ok(())
    })
}

fn scrape_page_data(document: &Html, url: &str) -> DocumentationData {
    let title_selector = Selector::parse("h1").unwrap(); // Selects <h1> tags
    let content_selector = Selector::parse("p, code").unwrap(); // Selects <p> and <code> tags

    let title = document
        .select(&title_selector)
        .next()
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_else(|| "No title found".to_string());

    let content_parts: Vec<String> = document
        .select(&content_selector)
        .flat_map(|element| element.text().map(|text| text.to_string()))
        .collect();
    let content = content_parts.join("\n");

    DocumentationData {
        title,
        url: url.to_string(),
        content,
    }
}

fn make_absolute_url(base_url: &str, href: &str) -> Option<String> {
    match Url::parse(base_url).and_then(|base| base.join(href)) {
        Ok(url) => Some(url.to_string()),
        Err(_) => None,
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn save_to_file(data: &Vec<DocumentationData>, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data).unwrap();
    std::fs::write(filename, json)?;
    println!("Data saved to {}", filename);
    Ok(())
}