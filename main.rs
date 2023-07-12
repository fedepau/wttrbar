use chrono::prelude::*;
use reqwest::blocking::get;
use serde_json::json;
use std::collections::HashMap;
use std::env;

const WEATHER_CODES: &[(i32, &str)] = &[
    (113, "󰖙 "),
    (116, "󰖕 "),
    (119, "󰖐 "),
    (122, "󰖕 "),
    (143, "󰖑 "),
    (176, "󰼳 "),
    (179, "󰖖 "),
    (182, "󰖘 "),
    (185, "󰖘 "),
    (200, "󰙾 "),
    (227, "󰖘 "),
    (230, "󰖘 "),
    (248, "󰖑 "),
    (260, "󰖑 "),
    (263, "󰖖 "),
    (266, "󰖖 "),
    (281, "󰼳 "),
    (284, "󰼳 "),
    (293, "󰖖 "),
    (296, "󰖖 "),
    (299, "󰖖 "),
    (302, "󰖖 "),
    (305, "󰖖 "),
    (308, "󰖖 "),
    (311, "󰖖 "),
    (314, "󰖖 "),
    (317, "󰖖 "),
    (320, "󰖘 "),
    (323, "󰖘 "),
    (326, "󰖘 "),
    (329, "󰖘 "),
    (332, "󰖘 "),
    (335, "󰖘 "),
    (338, "󰖘 "),
    (350, "󰖘 "),
    (353, "󰖖 "),
    (356, "󰖖 "),
    (359, "󰖖 "),
    (362, "󰖘 "),
    (365, "󰖘 "),
    (368, "󰖘 "),
    (371, "󰖘 "),
    (374, "󰖘 "),
    (377, "󰖘 "),
    (386, "󰙾 "),
    (389, "󰖘 "),
    (392, "󰖘 "),
    (395, "󰖘 "),
    (398, "󰖘 "),
    (401, "󰖘 "),
    (404, "󰖘 "),
    (407, "󰖘 "),
    (410, "󰖘 "),
    (413, "󰖘 "),
    (416, "󰖘 "),
    (419, "󰖘 "),
    (422, "󰖘 "),
    (425, "󰖘 "),
    (428, "󰖘 "),
    (431, "󰖘 "),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let main_indicator = match args.iter().position(|arg| arg == "--main-indicator") {
        Some(index) => args.get(index + 1).unwrap(),
        None => "temp_C",
    };

    let date_format = match args.iter().position(|arg| arg == "--date-format") {
        Some(index) => args.get(index + 1).unwrap(),
        None => "%Y-%m-%d",
    };

    let location = match args.iter().position(|arg| arg == "--location") {
        Some(index) => args.get(index + 1).unwrap(),
        None => "",
    };

    let hide_conditions = args.iter().any(|arg| arg == "--hide-conditions");

    let ampm = args.iter().any(|arg| arg == "--ampm");

    let fahrenheit = args.iter().any(|arg| arg == "--fahrenheit");

    let mut data = HashMap::new();

    let weather_url = if location.is_empty() {
        "https://wttr.in/?format=j1".to_string()
    } else {
        format!("https://wttr.in/{}?format=j1", location)
    };

    let weather = get(weather_url)
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();

    let current_condition = &weather["current_condition"][0];
    let indicator = current_condition[main_indicator].as_str().unwrap();
    let feels_like = if fahrenheit {
        current_condition["FeelsLikeF"].as_str().unwrap()
    } else {
        current_condition["FeelsLikeC"].as_str().unwrap()
    };
    let weather_code = current_condition["weatherCode"].as_str().unwrap();
    let weather_icon = WEATHER_CODES
        .iter()
        .find(|(code, _)| *code == weather_code.parse::<i32>().unwrap())
        .map(|(_, symbol)| symbol)
        .unwrap();
    let text = format!("{} {}", weather_icon, indicator);
    data.insert("text", text);

    let mut tooltip = format!(
        "<b>{}</b> {}°\n",
        current_condition["weatherDesc"][0]["value"]
            .as_str()
            .unwrap(),
        if fahrenheit {
            current_condition["temp_F"].as_str().unwrap()
        } else {
            current_condition["temp_C"].as_str().unwrap()
        },
    );
    tooltip += &format!("Feels like: {}°\n", feels_like);
    tooltip += &format!(
        "Wind: {}Km/h\n",
        current_condition["windspeedKmph"].as_str().unwrap()
    );
    tooltip += &format!(
        "Humidity: {}%\n",
        current_condition["humidity"].as_str().unwrap()
    );

    let now = Local::now();
    for (i, day) in weather["weather"].as_array().unwrap().iter().enumerate() {
        tooltip += "\n<b>";
        if i == 0 {
            tooltip += "Today, ";
        }
        if i == 1 {
            tooltip += "Tomorrow, ";
        }
        let date = NaiveDate::parse_from_str(day["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        tooltip += &format!("{}</b>\n", date.format(date_format));

        if fahrenheit {
            tooltip += &format!(
                "󱦲 {}° 󱦳 {}° ",
                day["maxtempF"].as_str().unwrap(),
                day["mintempF"].as_str().unwrap(),
            );
        } else {
            tooltip += &format!(
                "󱦲 {}° 󱦳 {}° ",
                day["maxtempC"].as_str().unwrap(),
                day["mintempC"].as_str().unwrap(),
            );
        };

        tooltip += &format!(
            "󰖜  {} 󰖛  {}\n",
            format_ampm_time(&day, "sunrise", ampm),
            format_ampm_time(&day, "sunset", ampm),
        );
        for hour in day["hourly"].as_array().unwrap() {
            if i == 0 && hour["time"].as_str().unwrap().parse::<u32>().unwrap() < now.hour() - 2 {
                continue;
            }

            let mut tooltip_line = format!(
                "{} {} {} {}",
                format_time(hour["time"].as_str().unwrap(), ampm),
                WEATHER_CODES
                    .iter()
                    .find(|(code, _)| *code
                        == hour["weatherCode"]
                            .as_str()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap())
                    .map(|(_, symbol)| symbol)
                    .unwrap(),
                if fahrenheit {
                    format_temp(hour["FeelsLikeF"].as_str().unwrap())
                } else {
                    format_temp(hour["FeelsLikeC"].as_str().unwrap())
                },
                hour["weatherDesc"][0]["value"].as_str().unwrap(),
            );
            if !hide_conditions {
                tooltip_line += format!(", {}", format_chances(hour)).as_str();
            }
            tooltip_line += "\n";
            tooltip += &tooltip_line;
        }
    }
    data.insert("tooltip", tooltip);

    let json_data = json!(data);
    println!("{}", json_data);
}

fn format_time(time: &str, ampm: bool) -> String {
    let hour = time.replace("00", "").parse::<i32>().unwrap();

    if ampm {
        let am_or_pm = if hour >= 12 { "pm" } else { "am" };
        let hour12 = if hour == 0 || hour == 12 {
            12
        } else {
            hour % 12
        };
        format!("{: <4}", format!("{}{}", hour12, am_or_pm))
    } else {
        format!("{:02}", hour)
    }
}

fn format_temp(temp: &str) -> String {
    format!("{: >3}°", temp)
}

fn format_chances(hour: &serde_json::Value) -> String {
    let chances: HashMap<&str, &str> = [
        ("chanceoffog", "Fog"),
        ("chanceoffrost", "Frost"),
        ("chanceofovercast", "Overcast"),
        ("chanceofrain", "Rain"),
        ("chanceofsnow", "Snow"),
        ("chanceofsunshine", "Sunshine"),
        ("chanceofthunder", "Thunder"),
        ("chanceofwindy", "Wind"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut conditions = vec![];
    for (event, name) in chances.iter() {
        if let Some(chance) = hour[event].as_str() {
            if let Ok(chance_value) = chance.parse::<u32>() {
                if chance_value > 0 {
                    conditions.push((name, chance_value));
                }
            }
        }
    }
    conditions.sort_by_key(|&(_, chance_value)| std::cmp::Reverse(chance_value));
    conditions
        .iter()
        .map(|&(name, chance_value)| format!("{} {}%", name, chance_value))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_ampm_time(day: &serde_json::Value, key: &str, ampm: bool) -> String {
    if ampm {
        day["astronomy"][0][key].as_str().unwrap().to_string()
    } else {
        NaiveTime::parse_from_str(day["astronomy"][0][key].as_str().unwrap(), "%I:%M %p")
            .unwrap()
            .format("%H:%M")
            .to_string()
    }
}
