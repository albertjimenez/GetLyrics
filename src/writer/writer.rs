use crate::model::data_model::{Lyric, Writer};
use log::{error, warn};
use std::fs;
use std::path::{Path, PathBuf};

impl Writer {
    /// Expected `.lrc` location for an audio file (same folder, same stem).
    /// Reuses the exact placement rule used by `write_lyric`.
    pub fn lyric_path_for_audio(audio_path: &Path) -> PathBuf {
        audio_path.with_extension("lrc")
    }

    /// True when the `.lrc` file exists next to the audio file.
    pub fn has_lyrics(audio_path: &Path) -> bool {
        Self::lyric_path_for_audio(audio_path).is_file()
    }

    /// Filter helper: keep only audio files that are missing their `.lrc`.
    pub fn missing_lyrics(files: &[PathBuf]) -> Vec<PathBuf> {
        files
            .iter()
            .filter(|path| !Self::has_lyrics(path))
            .cloned()
            .collect()
    }

    pub fn write_lyric(lyric: &Lyric) -> Option<PathBuf> {
        let lyrics = lyric.lyric.clone(); // or .to_owned()
        if lyrics.is_empty() {
            warn!("Lyrics were empty, skipping write operation.");
            return None;
        }
        // Single source of truth for `.lrc` placement (same folder, same stem).
        let full_path = Self::lyric_path_for_audio(&lyric.song.filepath);

        // Write lyrics to the new file
        if let Err(e) = fs::write(&full_path, lyrics) {
            error!("Failed to write lyric file: {}", e);
            return None;
        }
        Some(full_path)
    }
}
