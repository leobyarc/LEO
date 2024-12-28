use std::{env, process};
use crate::image::{Image, ImageGenerator, ImageRequest};
use crate::image_gen::ImageGen;
use crate::storage::Storage;
use crate::twitter::{ExtractedTweet, Twitter};
use crate::utils::generate_custom_image_path;
use crate::vision::{GoogleVision, GoogleVisionRequest};
use anyhow::Result;
use log::error;
use rig::completion::Prompt;
use rig::providers::openai;

// Main handler struct for processing tweets
pub struct Handler {
    translate_prompt: String,
    reply_text: String,
    // Storage for persisting processed tweet IDs
    storage: Storage,
    // Twitter client instance
    twitter: Twitter,
    // Maximum number of tweets to process
    max_tweets: i32,
}

impl Handler {
    // Initialize a new Handler instance with storage
    pub async fn initialize(storage: Storage) -> Result<Self> {
        let translate_prompt = env::var("TRANSLATE_PROMPT").unwrap_or_else(|err| {
            error!("Missing TRANSLATE_PROMPT {}", err);
            process::exit(1);
        });

        let reply_text = env::var("TWITTER_REPLY_TEXT").unwrap_or_else(|err| {
            error!("Missing TWITTER_REPLY_TEXT {}", err);
            process::exit(1);
        });

        Ok(Self {
            translate_prompt,
            reply_text,
            storage,
            twitter: Twitter::initialize().await?,
            max_tweets: 20,
        })
    }

    // Process new tweets mentioning the bot
    pub async fn handle_tweets(&mut self) -> Result<()> {
        // Search for tweets mentioning the bot
        let tweets = self
            .twitter
            .find_tweets(&format!("@{}", self.twitter.username), self.max_tweets, None, None)
            .await?;

        // Process each tweet
        for tweet in &tweets {
            // Extract tweet ID or skip if none
            let id = match &tweet.id {
                Some(id) => id.clone(),
                None => continue,
            };

            // Skip if tweet was already processed
            if self.storage.exists(id.clone()) {
                println!("Tweet {} already processed. Skipping", id.clone());
                continue;
            }

            // Handle tweet and track processed status
            if let Err(e) = self.process_single_tweet(tweet).await {
                println!("Error processing tweet {}: {:?}", id, e);
            } else {
                // Store processed tweet ID and save to file
                self.storage.add(id);
                self.storage.write_to_file()?;
            }
        }

        Ok(())
    }

    // Handle individual tweet processing
    async fn process_single_tweet(&self, tweet: &ExtractedTweet) -> Result<()> {
        // Get user profile information
        let profile = self
            .twitter
            .fetch_profile(tweet.username.clone().unwrap().as_str())
            .await?;

        // Skip if tweet is from the bot itself
        if profile.username == self.twitter.username {
            println!("Username is self. Skipping");
            return Ok(());
        }

        // Get user's avatar URL
        let avatar_url = match self.twitter.fetch_avatar(profile).await? {
            Some(url) => url,
            None => {
                println!("Avatar not found. Skipping");
                return Ok(());
            }
        };

        // Process image and generate response
        let image = Image::create_from_url(&avatar_url)?;
        let description = self.create_description(image)?;
        let translated_desc = self.convert_description(&description).await?;
        let image = self.produce_image(&translated_desc)?;

        // Send response tweet with generated image
        self.tweet_with_image(tweet, &image).await?;

        Ok(())
    }

    // Generate description using Google Vision API
    fn create_description(&self, image: Image) -> Result<String> {
        let vision = GoogleVision::initialize()?;
        let descs = vision.generate_description(GoogleVisionRequest { image, max_results: 10 })?;
        Ok(descs.join(","))
    }

    // Translate and optimize description using GPT-4
    async fn convert_description(&self, desc_string: &str) -> Result<String> {
        let client = openai::Client::from_env();
        let gpt4 = client.agent("gpt-4").build();
        let prompt_string = self.translate_prompt.replace("{}", &desc_string);
        let response: String = gpt4.prompt(&prompt_string).await?;

        Ok(response)
    }

    // Generate new image using DALL-E
    fn produce_image(&self, description: &str) -> Result<Image> {
        let image_gen = ImageGen::initialize()?;
        let image = image_gen.produce_image(ImageRequest {
            description: description.into(),
            width: 1792,
            height: 1024,
        })?;

        // Save generated image to disk
        let output_path = generate_custom_image_path();
        image.store(&output_path)?;
        println!("Saved image to {:?}", output_path);

        Ok(image)
    }

    // Send tweet with generated image as reply
    async fn tweet_with_image(&self, tweet: &ExtractedTweet, image: &Image) -> anyhow::Result<()> {
        let media_data = vec![(image.to_bytes(), "image/jpeg".to_string())];

        let reply_text = self.reply_text.replace("{}", tweet.username.clone().unwrap().as_str());
        let tweet_with_media = self.twitter.post_tweet(&reply_text, None, Some(media_data)).await?;

        println!("tweet_with_media {:#?}", tweet_with_media);
        Ok(())
    }
}