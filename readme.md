# Leo the Shiba Artist 🐕‍🎨

Leo the Shiba Artist is an AI creative agent powered by the Rig framework. It's a Shiba Inu who creates personalized artwork by analyzing Twitter users' avatars and transforming them into unique Shiba-style illustrations.

## Features

- 🎯 Automatic detection and analysis of Twitter profile pictures
- 🔍 Image feature extraction using Google Vision AI
- 🤖 Creative conceptualization through GPT-4
- 🎨 Personalized Shiba artwork generation via DALL-E
- 🐕 Integration of user avatar visual elements into Shiba character design

## Tech Stack

- Rust + Rig Framework
- OpenAI API (GPT-4 + DALL-E)
- Google Cloud Vision API

## How It Works

1. Receives Twitter @ mentions
2. Fetches user's profile picture
3. Analyzes image features using Google Vision AI
4. Generates creative concepts using GPT-4
5. Creates Shiba artwork through DALL-E
6. Replies to user with the generated artwork

## Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread", "full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.11.6"
thiserror = "2.0.9"
anyhow = "1.0"
dotenv = "0.15"
rig-core = "0.6.0"
ureq = { version = "2.8.0", features = ["json"] }
base64 = "0.22.1"
directories-next = "2.0.0"
uuid = { version = "1.5.0", features = ["v4"] }
agent-twitter-client = "0.1.2"
jsonwebtoken = "9.3.0"
```

## Environment Setup

```bash
TRANSLATE_PROMPT=  # Prompt text used for translation
TWITTER_REPLY_TEXT=  # Text used when replying to tweets
# Configure the OpenAI API key for interacting with the OpenAI API
OPENAI_API_KEY=  # API key for OpenAI
# Set the Twitter username for login
TWITTER_USERNAME=  # Username for the Twitter account
# Set the Twitter password for login
TWITTER_PASSWORD=  # Password for the Twitter account
# Set the Twitter email for login
TWITTER_EMAIL=  # Email associated with the Twitter account
# Set the Twitter 2fa for login
TWITTER_2FA_CODE=  # Two-factor authentication code for Twitter
```

## Quick Start

1. Clone the repository
```bash
git clone https://github.com/leobyarc/leo
cd leo
```

2. Configure environment variables
```bash
cp .env.example .env
# Edit .env file with your API keys
```

3. Run the project
```bash
cargo run
```

## Usage Example

Simply mention the bot on Twitter with an optional description:

```
@leobyarc Create a Shiba artwork for me!
```

Leo will automatically analyze your profile picture and create a Shiba artwork that incorporates elements from your avatar.

## Architecture

```
User Interaction Layer
├── Twitter API Handler
├── Response Generator
└── Image Publisher

AI Processing Layer
├── Image Analysis (Google Vision)
├── Concept Generation (GPT-4)
└── Art Creation (DALL-E)

Data Layer
└── Asset Storage
```

## Key Components

- **Twitter Listener**: Monitors mentions and processes requests
- **Vision Analyzer**: Extracts features from profile pictures
- **Creative Engine**: Generates artistic concepts and descriptions
- **Art Generator**: Creates final Shiba artwork
- **Storage Manager**: Handles data persistence and retrieval

## Error Handling

- Graceful handling of API rate limits
- Automatic retries for failed requests
- Error logging and monitoring
- User-friendly error messages

## Performance Considerations

- Parallel processing of requests
- Caching of frequently accessed data
- Optimized image processing pipeline
- Rate limiting implementation

## License

MIT License

## Future Improvements

- Support for additional art styles
- Enhanced background generation
- Animation capabilities
- Style transfer options
- Batch processing support

---

🐕 Powered by Rig Framework  
🎨 Every Avatar Gets Its Shiba Twin