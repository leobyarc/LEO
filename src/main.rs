use std::time::Duration;
use leo::{handler::Handler, storage::Storage};
use tokio::time::sleep;

// File path for persistent storage
const STORAGE_FILE: &'static str = "tweets.json";

// Main async function using tokio runtime
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Initialize the environment logger
    env_logger::init();

    // Load processed tweets from storage file
    let storage = Storage::read_from_file(STORAGE_FILE)?;

    // Create a new instance of Handler with storage
    let mut handler = Handler::initialize(storage).await?;

    // Infinite loop to continuously process tweets
    loop {
        // Print status message for each iteration
        println!("Starting a new iteration...");
        // Process tweets using the handler
        handler.handle_tweets().await?;
        // Sleep for 2 minutes before next iteration
        sleep(Duration::from_secs(1 * 60)).await;
    }
}