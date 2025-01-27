use std::io::{self, Write};

async fn connect(ip: &str, port: u16, password: &str) {
    let url = format!("http://{}:{}/auth", ip, port);

    println!("Connecting to the server at: {}", url);

    let client = reqwest::Client::new();
    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "password": password
        }))
        .send();

    match res.await {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                println!("Successfully connected to the server! Response: {:?}", res.text().await);
            } else {
                println!("Failed to connect to the server. Status code: {}", res.status());
            }
        }
        Err(e) => {
            println!("Failed to connect to the server. Error: {}", e);
        }
    }
}
#[tokio::main]
async fn main() {
    let mut ip = String::new();
    let mut port = String::new();
    loop {
        print!("Please enter an IP address: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut ip).unwrap();
        ip = ip.trim().to_string();

        if ip.parse::<std::net::IpAddr>().is_ok() || ip == "localhost" {
            print!("Please enter a port: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut port).unwrap();
            port = port.trim().to_string();

            if let Ok(port) = port.parse::<u16>() {
            println!("Valid IP address: {} and port: {}", ip, port);
            break;
            }
        } else {
            println!("The IP address is not valid. Please try again.");
        }
    }
    print!("Please enter the server password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    println!("Server password entered: {}", password);
    connect(&ip, port.parse().unwrap(), password).await;
}