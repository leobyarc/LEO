use std::{env, process};
use agent_twitter_client::{models::Profile, scraper::Scraper, search::SearchMode};
use log::error;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Main Twitter client struct
pub struct Twitter {
    // Username for the Twitter account
    pub username: String,
    // Password for the Twitter account
    pub password: String,
    // Email associated with the Twitter account
    pub email: String,
    // Instance of the Twitter scraper
    pub scraper: Scraper,
}

// Structure representing extracted tweet data
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedTweet {
    // Display name of the user
    pub name: Option<String>,
    // Twitter handle of the user
    pub username: Option<String>,
    // Unique identifier for the user
    pub user_id: Option<String>,
    // Content of the tweet
    pub text: Option<String>,
    // Timestamp of the tweet
    pub timestamp: Option<i64>,
    // Permanent URL to the tweet
    pub permanent_url: Option<String>,
    // Unique identifier for the tweet
    pub id: Option<String>,
}

impl Twitter {
    // Initialize a new Twitter client instance
    pub async fn initialize() -> Result<Self> {
        // Retrieve Twitter credentials from environment variables
        let username = env::var("TWITTER_USERNAME").unwrap_or_else(|err| {
            error!("Missing TWITTER_USERNAME {}", err);
            process::exit(1);
        });

        let password = env::var("TWITTER_PASSWORD").unwrap_or_else(|err| {
            error!("Missing TWITTER_PASSWORD {}", err);
            process::exit(1);
        });

        let email = env::var("TWITTER_EMAIL").unwrap_or_else(|err| {
            error!("Missing TWITTER_EMAIL {}", err);
            process::exit(1);
        });

        let two_factor_secret = env::var("TWITTER_2FA_CODE").unwrap_or_else(|err| {
            error!("Missing TWITTER_2FA_CODE {}", err);
            process::exit(1);
        });

        let two_factor_secret: Option<String> = if two_factor_secret.is_empty() {
            None
        } else {
            Some(two_factor_secret)
        };

        // Initialize and log in to Twitter
        let mut scraper = Scraper::new().await?;

        scraper
            .login(
                username.clone(),
                password.clone(),
                Some(email.clone()),
                two_factor_secret,
            )
            .await?;

        Ok(Self {
            username,
            password,
            email,
            scraper,
        })
    }

    // Search for tweets matching a query
    pub async fn find_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: Option<SearchMode>,
        cursor: Option<String>,
    ) -> Result<Vec<ExtractedTweet>> {
        // Execute tweet search
        let tweets = self
            .scraper
            .search_tweets(query, max_tweets, search_mode.unwrap_or(SearchMode::Latest), cursor)
            .await?;

        // Convert tweets to ExtractedTweet format
        let extracted_tweets: Vec<ExtractedTweet> = tweets
            .tweets
            .iter()
            .map(|tweet| ExtractedTweet {
                name: tweet.name.clone(),
                username: tweet.username.clone(),
                user_id: tweet.user_id.clone(),
                text: tweet.text.clone(),
                timestamp: tweet.timestamp,
                permanent_url: tweet.permanent_url.clone(),
                id: tweet.id.clone(),
            })
            .collect();

        Ok(extracted_tweets)
    }

    // Retrieve user profile information
    pub async fn fetch_profile(&self, username: &str) -> Result<Profile> {
        let profile = self.scraper.get_profile(username).await?;
        Ok(profile)
    }

    // Retrieve user's avatar URL from profile
    pub async fn fetch_avatar(&self, profile: Profile) -> Result<Option<String>> {
        Ok(profile.profile_image_url)
    }

    // Post a new tweet with optional media
    pub async fn post_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        let tweet_with_media = self.scraper.send_tweet(text, reply_to, media_data).await?;
        Ok(tweet_with_media)
    }
}
