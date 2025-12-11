use std::fmt::format;

use regex::Regex;
use serde::Deserialize;

use crate::model::data_model::{Lyric, SongMetadata};
use crate::traits::traits::LyricIface;

#[derive(Deserialize)]
struct ApiResponse {
    lyrics: String,
}
pub struct LyricApi {}

impl LyricApi {
    fn sanitize_lyrics(lyrics: &str) -> String {

        // Replace \r\n and multiple \n with a single newline
        let newline_regex = Regex::new(r"\r\n|\n+").unwrap();
        let lyrics = newline_regex.replace_all(lyrics, "\n");

        // Remove excessive spaces
        let space_regex = Regex::new(r" +").unwrap();
        let lyrics = space_regex.replace_all(&lyrics, " ");

        // Trim leading and trailing whitespace
        lyrics.trim().to_string()
    }
    pub fn new() -> Self {
        LyricApi {}
    }
}
impl LyricIface for LyricApi {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<Lyric, String> {
        let url = format(format_args!("https://api.lyrics.ovh/v1/{}/{}", &song_metadata.artist, &song_metadata.title));
        let response = reqwest::blocking::get(&url)
            .map_err(|e| format!("Network request failed: {}", e))?;

        if !response.status().is_success() {
            return Err("Lyric not found with LyricsAPI".to_owned());
        } else {
            let api_response: Result<ApiResponse, _> = response.json();
            if api_response.is_ok() {
                let lyrics = Self::sanitize_lyrics(&api_response.unwrap().lyrics);
                return Ok(Lyric { lyric: lyrics, song: song_metadata.song.clone() });
            }
        }
        Err("Lyric API is down".to_owned())
    }
}