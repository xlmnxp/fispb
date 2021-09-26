use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let links: Vec<&str> = vec![
        "https://speedtest.saudi.net.sa:8080/speedtest/upload.php",
        "http://speedtest.saudi.net.sa.prod.hosts.ooklaserver.net:8080/speedtest/upload.php"
    ];
    let delay_in_seconds = 5;

    loop {
        for link in &links {
            println!("status of {}: {}", link, reqwest::blocking::get(*link)?.status());
        }
        sleep(Duration::from_secs(delay_in_seconds));
    }
}
