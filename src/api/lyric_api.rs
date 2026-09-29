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
        let url = format(format_args!(
            "https://api.lyrics.ovh/v1/{}/{}",
            &song_metadata.artist, &song_metadata.title
        ));
        let response =
            reqwest::blocking::get(&url).map_err(|e| format!("Network request failed: {}", e))?;

        if !response.status().is_success() {
            return Err("Lyric not found with LyricsAPI".to_owned());
        } else {
            let api_response: Result<ApiResponse, _> = response.json();
            if api_response.is_ok() {
                let lyrics = Self::sanitize_lyrics(&api_response.unwrap().lyrics);
                return Ok(Lyric {
                    lyric: lyrics,
                    song: song_metadata.song.clone(),
                });
            }
        }
        Err("Lyric API is down".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_lyric_api() {
        let _api = LyricApi::new();
    }

    #[test]
    fn test_sanitize_lyrics_trims_leading_and_trailing_whitespace() {
        let lyrics = "   Some lyrics   ";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("Some lyrics", sanitized);
    }

    #[test]
    fn test_sanitize_lyrics_replaces_windows_newlines_with_single_newline() {
        let lyrics = "First line\r\nSecond line\r\nThird line";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("First line\nSecond line\nThird line", sanitized);
    }

    #[test]
    fn test_sanitize_lyrics_collapses_multiple_newlines() {
        let lyrics = "First line\n\n\nSecond line\n\nThird line";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("First line\nSecond line\nThird line", sanitized);
    }

    #[test]
    fn test_sanitize_lyrics_collapses_multiple_spaces() {
        let lyrics = "First    line with     many      spaces";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("First line with many spaces", sanitized);
    }

    #[test]
    fn test_sanitize_lyrics_handles_mixed_whitespace() {
        let lyrics = "   First    line\r\n\r\nSecond     line\n\n\nThird   line   ";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("First line\n\nSecond line\nThird line", sanitized);
    }

    #[test]
    fn test_sanitize_lyrics_returns_empty_string_for_whitespace_only_input() {
        let lyrics = "     \n\n\r\n     ";

        let sanitized = LyricApi::sanitize_lyrics(lyrics);

        assert_eq!("", sanitized);
    }

    #[test]
    fn test_api_response_can_hold_lyrics() {
        let response = ApiResponse {
            lyrics: String::from("Some lyrics"),
        };

        assert_eq!("Some lyrics", response.lyrics);
    }
}