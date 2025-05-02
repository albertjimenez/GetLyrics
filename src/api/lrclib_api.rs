use log::warn;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::model::data_model::{Lyric, SongMetadata};
use crate::traits::traits::LyricIface;

#[derive(Debug, Deserialize)]
struct LrcLibResponse {
    #[serde(rename = "plainLyrics")]
    plain_lyrics: String,
    #[serde(rename = "syncedLyrics")]
    synced_lyrics: Option<String>,
}
pub struct LrcLibAPI {
    karaoke: bool,
}

impl LrcLibAPI {
    pub fn new_karaoke_lyrics() -> Self {
        LrcLibAPI { karaoke: true }
    }
    pub fn new_plain_lyrics() -> Self {
        LrcLibAPI { karaoke: false }
    }
}
impl LyricIface for LrcLibAPI {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<Lyric, String> {
        let base_url = "https://lrclib.net/api/get";
        let params = [
            ("track_name", song_metadata.title.as_str()),
            ("artist_name", song_metadata.artist.as_str()),
            ("album_name", song_metadata.album_title.as_str()),
            ("duration", &song_metadata.duration.unwrap_or(0).to_string()),
        ];
        let client = Client::builder()
            .user_agent("https://github.com/albertjimenez/GetLyrics")
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        let response = client.get(base_url)
            .query(&params)

            .send()
            .map_err(|e| format!("Request error: {}", e))?;

        match response.status().as_u16() {
            200 => {
                let data: LrcLibResponse = response
                    .json()
                    .map_err(|e| format!("Failed to parse response JSON: {}", e))?;
                let mut lyrics = data.plain_lyrics;
                if data.synced_lyrics.is_none() && self.karaoke {
                    warn!("Falling back to traditional lyric since no synced lyric was found for {}", &song_metadata.title);
                }
                if data.synced_lyrics.is_some() && self.karaoke {
                    lyrics = data.synced_lyrics.unwrap();
                }
                let lyric = Lyric { lyric: lyrics, song: song_metadata.song.to_owned() };
                return Ok(lyric);
            }
            404 => Err("Lyrics not found.".to_string()),
            code => Err(format!("Unexpected status code: {}", code)),
        }
    }
}
