use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use log::error;

use crate::model::data_model::AudioExtensions::{
    AAC, AIFF, FLAC, M4A, MP3, OGG, UNKNOWN, WAV, WMA,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
pub struct Lyric {
    pub lyric: String,
    pub song: Song,
}
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
pub struct Song {
    pub filename: String,
    pub extension: AudioExtensions,
    pub filepath: PathBuf,
}
impl Song {
    pub fn new(filepath: &str) -> Option<Song> {
        let filepath_as_path = filepath.as_ref();
        let extension = AudioExtensions::get_extension_by_filepath(filepath_as_path);
        if extension == UNKNOWN {
            error!("Invalid file/extension on filepath => {}", &filepath);
            return None;
        }
        let filepath_as_path = filepath_as_path.to_owned();
        let filename = String::from(filepath_as_path.file_name().unwrap().to_str().unwrap());

        Some(Song {
            filename,
            extension,
            filepath: filepath_as_path,
        })
    }
    pub fn empty() -> Song {
        let (filename, extension, filepath) = (String::new(), MP3, PathBuf::new());
        Song {
            filename,
            extension,
            filepath,
        }
    }
    pub fn is_file(&self) -> bool {
        match self.filepath.try_exists() {
            Ok(true) => self.filepath.is_file(),
            Ok(false) | Err(_) => false,
        }
    }
}
impl Display for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "filename: {}, extension: {}, filepath: {}",
            self.filename,
            self.extension,
            self.filepath.to_str().unwrap_or("")
        )
    }
}
#[derive(Debug)]
pub struct SongMetadata {
    pub song: Song,
    pub artist: String,
    pub title: String,
    pub album_title: String,
    pub duration: Option<u16>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum AudioExtensions {
    MP3,
    OGG,
    M4A,
    FLAC,
    WAV,
    AIFF,
    WMA,
    AAC,
    UNKNOWN,
}
impl AudioExtensions {
    pub(crate) fn get_extension(&self) -> &'static str {
        match &self {
            MP3 => "mp3",
            OGG => "ogg",
            M4A => "m4a",
            FLAC => "flac",
            WAV => "wav",
            AIFF => "aiff",
            WMA => "wma",
            AAC => "aac",
            UNKNOWN => "unknown",
        }
    }
    fn get_extension_as_str(filepath: &Path) -> String {
        let cloned = filepath.to_owned();
        let os_string = cloned
            .extension()
            .unwrap_or(OsStr::new(UNKNOWN.get_extension()))
            .to_ascii_lowercase();
        String::from(os_string.to_str().unwrap_or(""))
    }
    pub fn get_extension_by_filepath(filepath: &Path) -> AudioExtensions {
        let extensions = vec![MP3, OGG, M4A, FLAC, WAV, AIFF, WMA, AAC];
        let current_extension = Self::get_extension_as_str(filepath);
        let result = extensions
            .into_iter()
            .find(|an_extension| current_extension.as_str() == an_extension.get_extension());

        result.unwrap_or_else(|| UNKNOWN)
    }
}
impl Display for AudioExtensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_extension())
    }
}

pub struct Writer;
