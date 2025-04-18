use crate::model::data_model::SongMetadata;

pub trait LyricScraper {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<String, String>;
}