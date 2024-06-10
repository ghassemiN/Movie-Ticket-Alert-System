extern crate dotenv;

use dotenv::dotenv;
use std::env;
use reqwest::blocking::get;
use scraper::{Html, Selector};


fn main()  {

    // Load the .env file
    dotenv().ok();

    let my_url = "https://www.paribucineverse.com/animasyon-filmleri/ters-yuz-2-filmi-izle"; // You should change the url to any movie page you want to track from paribucineverse.com
 
    // Extract the movie name from the URL 
    let movie_name = my_url.split('/').last().unwrap().trim_end_matches("-filmi-izle");

    let response = get(my_url).expect("Failed to send request");

    let body = response.text().expect("Failed to read response body");

    let document = Html::parse_document(&body);

    let selector = Selector::parse(".cgv-btn").expect("Failed to parse CSS selector");

    // Iterate over all elements that match the selector
    for element in document.select(&selector) {
        if element.text().collect::<String>() == "Hemen Bilet Al" {
            let message = format!("{} is now available", movie_name);
            send_to_bot(message);
            break;
        }
    }

}


// A function that sends a message to the bot
fn send_to_bot(message: String) {
    let bot_token = env::var("BOT_TOKEN").expect("bot_token must be set");
    let chat_id = env::var("CHAT_ID").expect("chat_id must be set");
    let bot_api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    let client = reqwest::blocking::Client::new();
    
    let params = [
        ("chat_id", chat_id),
        ("text", message),
    ];
    
    let response = client.post(&bot_api_url)
        .form(&params)
        .send()
        .expect("Failed to send request");
    
    if response.status().is_success() {
        println!("Message sent successfully");
    } else {
        println!("Failed to send message");
    }
}