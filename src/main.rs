use prettytable::{Table, row};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::io::{self, BufRead};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize)]
struct Repository {
    stargazers_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read repositories from stdin
    let stdin = io::stdin();
    let repositories: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    if repositories.is_empty() {
        println!("No repositories provided on stdin.");
        return Ok(());
    }

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let markdown_format = args.iter().any(|arg| arg == "--markdown" || arg == "-m");

    // Create a table for output
    let mut table = Table::new();
    table.add_row(row!["Repository", "Stars", "URL"]);

    // Setup HTTP client with headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("GitHub-Stars-Fetcher"));

    // Add your GitHub token if you have one to avoid rate limits
    // headers.insert(
    //     "Authorization",
    //     HeaderValue::from_str(&format!("token {}", "YOUR_GITHUB_TOKEN"))?,
    // );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Fetch star counts for each repository
    let mut results = Vec::new();

    for repo in repositories {
        // Respect GitHub API rate limits
        sleep(Duration::from_millis(700)).await;

        println!("Fetching data for: {}", repo);

        let url = format!("https://api.github.com/repos/{}", repo);

        match client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Repository>().await {
                        Ok(data) => {
                            results.push((
                                repo.clone(),
                                data.stargazers_count,
                                format!("https://github.com/{}", repo),
                            ));
                        }
                        Err(e) => {
                            results.push((repo.clone(), 0, format!("Error parsing JSON: {}", e)));
                        }
                    }
                } else {
                    results.push((
                        repo.clone(),
                        0,
                        format!("HTTP Error: {}", response.status()),
                    ));
                }
            }
            Err(e) => {
                results.push((repo.clone(), 0, format!("Request Error: {}", e)));
            }
        }
    }

    // Sort by stars (descending)
    results.sort_by(|a, b| b.1.cmp(&a.1));

    if markdown_format {
        // Print results in markdown format
        for (repo, stars, url) in results {
            println!("- [{}]({}) ({}⭐)", repo, url, stars);
        }
    } else {
        // Add results to the table
        for (repo, stars, url) in results {
            table.add_row(row![repo, stars, url]);
        }

        // Print the table
        table.printstd();
    }

    Ok(())
}
