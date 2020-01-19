use std::{thread, time};
use chrono::{Utc, Duration, DateTime};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use reqwest;
use serde_json::Value;

use rppal::gpio::Gpio;

use config;
use std::collections::HashMap;

fn main() {
    let mut last_time = Utc::now().with_timezone(&Tokyo) - Duration::minutes(61);

    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(23).unwrap().into_input();

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("/opt/MimamoriSettings.toml")).unwrap();
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();
    let url: String = settings_map["url"].parse().unwrap();
    let api_key: String = settings_map["api_key"].parse().unwrap();

    loop {
        if pin.is_high() {
            let json_value = callapi(&url, &api_key, last_time);
            last_time = DateTime::parse_from_rfc3339(&json_value["last_time"].to_string().replace("\"", "")).unwrap().with_timezone(&Tokyo);
            thread::sleep(time::Duration::from_secs(600));
        }
    }
}


fn callapi(url: &str, api_key: &str, last_time: DateTime<Tz>) -> Value {

    let mut params = HashMap::new();
    params.insert("last_time", last_time.to_rfc3339());

    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header("x-api-key", api_key)
        .json(&params)
        .send();
    let json_value: Value = serde_json::from_str(&res.unwrap().text().unwrap()).unwrap();
    json_value
}