use reqwest::Client;
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let target_url = "https://google.com"; // particular endpoint you wanna hit multiple request with.
    let concurrent_requests = 100; // Number of concurrent requests 

    let client = Client::new(); //for HTTP

    let mut tasks = Vec::with_capacity(concurrent_requests);

    for _ in 0..concurrent_requests {
        let url = target_url.to_string();
        let client = client.clone();

        let task = task::spawn(async move {
            loop {
                // Send an HTTP GET request 
                match client.get(&url).send().await {
                    Ok(response) => {
                        println!("Response: {}", response.status());
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }

                // Wait for a short period before sending the next request
                sleep(Duration::from_millis(100)).await; 
            }
        });
            //pushing all the responses and error into tasks vector.
        tasks.push(task);
    }

    // Await all tasks (they will run indefinitely)
    for task in tasks {
        task.await.unwrap();
    }
}