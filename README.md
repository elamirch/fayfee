# FayFee

A simple Rust-based feed aggregator that fetches the latest headlines from [NewsAPI](https://newsapi.org/) and posts them to your Telegram channel. Future roadmap includes AI-powered interpretations of each article.

---

## 🔧 Features

- Fetch top headlines from NewsAPI
- Post articles to a Telegram channel
- 🛠️ Built-in library for AI-powered message interpretations (coming soon)
- Easy cronjob setup for daily automation

---

## 🚀 Prerequisites

1. **Rust & Cargo**  
   - Install from [rustup.rs](https://rustup.rs/).
2. **Telegram Bot**  
   - Create a bot via [@BotFather](https://t.me/BotFather).  
   - Save your **Bot Token**.
3. **Telegram Channel**  
   - Create a public or private channel.  
   - Add your bot as an administrator.
4. **NewsAPI Key**  
   - Sign up at [newsapi.org](https://newsapi.org/).  
   - Copy your **API Key**.

---

## ⚙️ Installation

```bash
git clone https://github.com/elamir/fayfee.git
cd fayfee
cargo build --release
```

---

## 🔑 Configuration

Rename the `env.example` file in project root to `.env`:

```dotenv
NEWSAPI_KEY=your_key_here
NEWSAPI_ENDPOINT="https://newsapi.org/v2/top-headlines"

METIS_SESSION_ID=your_session_id_here
METIS_API_KEY=your_key_here

TELEGRAM_API=your_key_here
TELEGRAM_CHANNEL_ID=your_channel_id_here
```

### How to find your Channel ID

1. Post any message in your channel (e.g. “Hello!”).  
2. Use Telegram’s **getUpdates** endpoint:

   ```bash
   curl -s "https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates" \
     | jq '.result[].message'
   ```

3. Look for the `"chat":{"id": ...}` field.  
   - Channel IDs are negative integers, e.g. `-1001234567890`.

---

## 💡 Usage

Run the binary directly:

```bash
./target/release/fayfee
```

You should see logs like:

```
FayFee is running...
Date: 2025-04-24
Article being processed: some_article_title
All articles sent!
```

---

## 🗓️ Automate with Cron

Add to your crontab to run daily (e.g. at 8 AM):

```cron
0 8 * * * /path/to/fayfee --env-file /path/to/.env >> /var/log/fayfee.log 2>&1
```

---

## 🤖 AI-Powered Interpretations (Roadmap)

We include a library (`metisai`) that will let you ask for custom interpretations of the news. Example usage (coming soon!):

---

## 📋 Roadmap

- [x] Fetch & post headlines  
- [x] .env configuration support  
- [ ] AI-driven interpretations (Q2 2025)  
- [ ] Multi-source support (RSS, other APIs)  
- [ ] Web dashboard & metrics

---

## 📝 License

This project is distributed under the MIT License.  
```
