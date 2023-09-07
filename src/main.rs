extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate csv;

use std::env;
use csv::{ReaderBuilder, WriterBuilder};
use chrono::Utc;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug, Clone)]
struct Log {
    id: String,
    recipient_address: String,
    status: String,
    created: i64,
    email_id: String,
    email_name: String,
    message: String,
    #[serde(skip_deserializing)]
    created_readable: Option<String>,
}

// Now define LogCsv and its implementation
#[derive(Serialize)]
struct LogCsv {
    id: String,
    recipient_address: String,
    status: String,
    created_readable: String,
    email_id: String,
    email_name: String,
    message: String,
}

impl From<&Log> for LogCsv {
    fn from(log: &Log) -> Self {
        LogCsv {
            id: log.id.clone(),
            recipient_address: log.recipient_address.clone(),
            status: log.status.clone(),
            created_readable: log.created_readable.clone().unwrap_or_else(|| "N/A".to_string()),
            email_id: log.email_id.clone(),
            email_name: log.email_name.clone(),
            message: log.message.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    success: bool,
    logs: Vec<Log>,
}

fn fetch_logs_by_email(email: &str, api_key: &str) -> Result<Vec<Log>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.sendwithus.com/api/v1/customers/{}/logs", email);
    
    println!("Fetching logs for {}", email);
    let raw_response = client.get(&url).basic_auth(api_key.to_string(), None::<&str>).send()?;
    let raw_text = raw_response.text()?;
    // println!("Raw API response: {}", raw_text);

    let response: Result<ApiResponse, serde_json::Error> = serde_json::from_str(&raw_text);
    match response {
        Ok(mut api_response) => {
            if api_response.success {
                // Convert UNIX timestamps to human-readable date-time strings
                for log in &mut api_response.logs {
                    let dt = chrono::TimeZone::from_utc_datetime(&Utc, &NaiveDateTime::from_timestamp_opt(log.created, 0).unwrap());
                    log.created_readable = Some(dt.to_rfc3339());
                }
                Ok(api_response.logs)
            } else {
                println!("API returned false for 'success' field");
                Ok(vec![])
            }
        },
        Err(e) => {
            println!("Failed to deserialize API response: {:?}", e);
            Err(e.into())
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("SWU_KEY").expect("SWU_KEY must be set");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: swu_fool <input_file.csv>");
        return Ok(());
    }
    let input_file = &args[1];

    let mut reader = ReaderBuilder::new()
        .from_path(input_file)?;

    let mut writer = WriterBuilder::new()
        .from_path("output.csv")?;

    for result in reader.records() {
        let record = result?;
        let email = &record[0];
        if let Ok(logs) = fetch_logs_by_email(email, &api_key) {
            for log in logs {
                let log_csv = LogCsv::from(&log);
                writer.serialize(log_csv)?;
            }
        } else {
            println!("Failed to fetch logs for {}", email);
        }
    }

    println!("CSV file written");

    Ok(())
}

