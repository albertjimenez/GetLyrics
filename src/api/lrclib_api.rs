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

        let response = client
            .get(base_url)
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
                    warn!(
                        "Falling back to traditional lyric since no synced lyric was found for {}",
                        &song_metadata.title
                    );
                }
                if data.synced_lyrics.is_some() && self.karaoke {
                    lyrics = data.synced_lyrics.unwrap();
                }
                let lyric = Lyric {
                    lyric: lyrics,
                    song: song_metadata.song.to_owned(),
                };
                Ok(lyric)
            }
            404 => Err("Lyrics not found.".to_string()),
            code => Err(format!("Unexpected status code: {}", code)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_karaoke_lyrics_enables_karaoke_mode() {
        let api = LrcLibAPI::new_karaoke_lyrics();

        assert!(api.karaoke);
    }

    #[test]
    fn test_new_plain_lyrics_disables_karaoke_mode() {
        let api = LrcLibAPI::new_plain_lyrics();

        assert!(!api.karaoke);
    }

    #[test]
    fn test_lrc_lib_response_can_hold_plain_lyrics_without_synced_lyrics() {
        let response = LrcLibResponse {
            plain_lyrics: String::from("Plain lyrics"),
            synced_lyrics: None,
        };

        assert_eq!("Plain lyrics", response.plain_lyrics);
        assert_eq!(None, response.synced_lyrics);
    }

    #[test]
    fn test_lrc_lib_response_can_hold_synced_lyrics() {
        let response = LrcLibResponse {
            plain_lyrics: String::from("Plain lyrics"),
            synced_lyrics: Some(String::from("[00:01.00] Synced lyrics")),
        };

        assert_eq!("Plain lyrics", response.plain_lyrics);
        assert_eq!(Some(String::from("[00:01.00] Synced lyrics")), response.synced_lyrics);
    }
}