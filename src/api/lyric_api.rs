use std::fmt::format;
use log::{warn};
use crate::model::data_model::{Lyric, SongMetadata};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    lyrics: String,
}
pub struct LyricApi{}
impl LyricApi {
    pub  fn get_lyrics(song_metadata: &SongMetadata) -> Option<Lyric> {
        let url = format(format_args!("https://api.lyrics.ovh/v1/{}/{}", &song_metadata.artist, &song_metadata.title));
        let response =  reqwest::blocking::get(url).unwrap();
        if !response.status().is_success() {
            warn!("Could not fetch the lyrics. Main method will attempt to retrieve them with scrapping.");
            return None
        }
        else {
            let api_response: Result<ApiResponse, _> = response.json();
            if api_response.is_ok() {
                let lyrics = Self::sanitize_lyrics(&api_response.unwrap().lyrics);
                return Some(Lyric{lyric: lyrics, song: song_metadata.song.clone()})
            }

        }
        None
    }
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
}