use std::fs;
use std::path::PathBuf;
use log::{warn};
use crate::model::data_model::{Lyric, Writer};

impl Writer {
    pub fn write_lyric(lyric: &Lyric) -> Option<PathBuf> {
        let parent_filepath = lyric.song.filepath.parent()?;
        let lyrics = lyric.lyric.clone(); // or .to_owned()
        if lyrics.is_empty() {
            warn!("Lyrics were empty, skipping write operation.");
            return None
        }
        let original_filename = lyric.song.filename.clone();

        let mut new_filename = PathBuf::from(&original_filename);
        new_filename.set_extension("lrc");

        let full_path = parent_filepath.join(new_filename);

        // Write lyrics to the new file
        if let Err(e) = fs::write(&full_path, lyrics) {
            eprintln!("Failed to write lyric file: {}", e);
            return None;
        }
        Some(full_path)
    }
}