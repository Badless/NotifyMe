use std::{process::Command, env};
use chrono::prelude::*;
use tokio::time::sleep;

async fn notify(title: &str, message: &str) {
    Command::new("notify-send").arg(title).arg(message).output().expect("error");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Error! it's taking 3 arguments:
                 \nnotify-me \"Title\" \"Message\" Date
                 \nexample:\nnotify-me \"Test\" \"Hello World\" 25/05/2023");
        return;
    }

    let input_date = &args[3];

    let parsed_date = NaiveDate::parse_from_str(input_date, "%d/%m/%Y");

    match parsed_date {
        Ok(date) => {
            let now = Local::now().naive_local();
            let target_datetime = date.and_hms(0, 0, 0);
            let duration = target_datetime.signed_duration_since(now);

            if duration > chrono::Duration::zero() {
                println!("Scheduling the message to be sent at {}", target_datetime);
                sleep(duration.to_std().unwrap()).await;
                notify(args[1].as_str(), args[2].as_str()).await;
            } else {
                println!("The specified date has already passed. Please provide a future date.");
            }
        }
        Err(_) => {
            println!("Invalid date format. Please provide a date in the format DD/MM/YYYY");
        }
    }
}
