use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use inquire::Text;
use rand::{Rng, thread_rng};
use reqwest::{Client, Error};

use crate::utils::{generate_random_digits, generate_random_string};

mod utils;

async fn run(user_id: &String, client: &Client) -> Result<(), Error> {
    let install_id = generate_random_string(22);

    let mut data = HashMap::new();
    data.insert("key", format!("{}=", generate_random_string(43)));
    data.insert("install_id", install_id.clone());
    data.insert("fcm_token", format!("{}:APA91b{}", &install_id, generate_random_string(134)));
    data.insert("referrer", user_id.clone());
    data.insert("warp_enabled", "false".to_string());
    data.insert("tos", Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
    data.insert("type", "Android".to_string());
    data.insert("locale", "en_US".to_string());


    let response =
        client.post(format!("https://api.cloudflareclient.com/v0a{}/reg", generate_random_digits(3)))
            .json(&data)
            .send().await;

    if let Err(why) = response {
        return Err(why);
    }

    if let Err(why) = response.unwrap().error_for_status() {
        return Err(why);
    }

    return Ok(());
}

#[tokio::main]
async fn main() {
    let user_id_prompt = Text::new("WARP User ID?").prompt();

    if let Err(why) = &user_id_prompt {
        eprintln!("Error: {}", why);
        std::process::exit(0);
    }

    let user_id = user_id_prompt.unwrap();

    let client = reqwest::Client::new();

    loop {
        match run(&user_id, &client).await {
            Ok(_) => println!("Success! Granted 1GB of WARP+ Data!"),
            Err(why) => eprintln!("Error: {}, trying again...", why)
        }

        thread::sleep(Duration::from_secs(thread_rng().gen_range(30..45)));
    }
}
