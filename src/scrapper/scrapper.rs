use reqwest::blocking::Client;
use scraper::{Html, Selector};

use crate::model::data_model::{LyricScraperV1, SongMetadata};
use crate::traits::traits::LyricScraper;

impl LyricScraperV1 {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0")
            .build()
            .expect("Failed to build HTTP client");
        LyricScraperV1 { client }
    }

    fn slugify(input: &str) -> String {
        input
            .trim()
            .to_lowercase()
            .replace(&[' ', '_', '.', ',', '!', '?', ':', ';'][..], "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }
}

impl LyricScraper for LyricScraperV1 {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<String, String> {
        let (artist, title) = (&song_metadata.artist, &song_metadata.title);
        let artist_slug = Self::slugify(artist);
        let title_slug = Self::slugify(title);
        let url = format!("https://www.letras.com/{}/{}/", artist_slug, title_slug);

        let html = self.client.get(&url)
            .send()
            .map_err(|e| format!("Request failed: {}", e))?
            .text()
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        let document = Html::parse_document(&html);
        let selector = Selector::parse("div.lyric-original p")
            .map_err(|e| format!("Selector parse error: {}", e))?;

        let lyrics = document.select(&selector)
            .map(|p| p.text().collect::<Vec<_>>().join("\n"))
            .collect::<Vec<String>>()
            .join("\n");

        if lyrics.trim().is_empty() {
            Err("Lyrics not found".to_string())
        } else {
            Ok(lyrics)
        }
    }
}