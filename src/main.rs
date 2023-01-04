use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut curl_handle = Easy::new();
    let mut response = Vec::new();

    curl_handle
        .url(r#"https://wttr.in/StellenboschFarms?format=j1"#)
        .unwrap();

    {
        let mut transfer = curl_handle.transfer();

        transfer
            .write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    // curl_handle
    //     .write_function(|data| {
    //         stdout().write_all(data).unwrap();
    //         Ok(data.len())
    //     })
    //     .unwrap();

    curl_handle.perform().unwrap();

    let response = std::str::from_utf8(&response).expect("Invalid UTF8");

    let json: serde_json::Value = serde_json::from_str(response).unwrap();

    // println!("{}", json["current_condition"]);

    let info = WeatherInfo::new(json);

    let l1 = format!(
        "{}°C | {}% | {}hPa",
        info.temp, info.humidity, info.pressure,
    );

    let l2 = format!("{}", info.flavour_text);

    println!("{:^24}\n{:^24}", l1, l2);
}

#[derive(Debug)]
struct WeatherInfo {
    temp: String,
    humidity: String,
    pressure: String,
    wind_direction: String,
    wind_speed: String,
    precip: String,
    flavour_text: String,
}
impl WeatherInfo {
    fn new(value: serde_json::Value) -> Self {
        let temp: String =
            serde_json::from_value(value["current_condition"][0]["FeelsLikeC"].clone()).unwrap();

        let humidity: String =
            serde_json::from_value(value["current_condition"][0]["humidity"].clone()).unwrap();
        let pressure: String =
            serde_json::from_value(value["current_condition"][0]["pressure"].clone()).unwrap();
        let wind_direction: String =
            serde_json::from_value(value["current_condition"][0]["winddir16Point"].clone())
                .unwrap();
        let wind_speed: String =
            serde_json::from_value(value["current_condition"][0]["windspeedKmph"].clone()).unwrap();
        let precip: String =
            serde_json::from_value(value["current_condition"][0]["precipMM"].clone()).unwrap();
        let flavour_text: String = serde_json::from_value(
            value["current_condition"][0]["weatherDesc"][0]["value"].clone(),
        )
        .unwrap();

        Self {
            temp,
            humidity,
            pressure,
            wind_direction,
            wind_speed,
            precip,
            flavour_text,
        }
    }
}
