use crate::model::data_model::{Lyric, SongMetadata};

pub trait LyricIface {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<Lyric, String>;
    // fn new() -> Self;
}