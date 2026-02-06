use crate::model::data_model::{Lyric, SongMetadata};
use anyhow::Result;
use std::path::Path;

pub trait LyricIface {
    fn fetch_lyrics(&self, song_metadata: &SongMetadata) -> Result<Lyric, String>;
}

pub trait ProcessPolicy: Send + Sync {
    fn should_process(&self, path: &Path) -> Result<bool>;
}
