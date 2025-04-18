
#[cfg(test)]
mod data_model_tests {
    use std::path::PathBuf;
    use crate::model::data_model::{AudioExtensions, Song};

    #[test]
    fn test_audio_extensions_get_extension() {
        assert_eq!(AudioExtensions::MP3.get_extension(), "mp3");
        assert_eq!(AudioExtensions::OGG.get_extension(), "ogg");
        assert_eq!(AudioExtensions::FLAC.get_extension(), "flac");
        assert_eq!(AudioExtensions::UNKNOWN.get_extension(), "unknown");
    }

    #[test]
    fn test_audio_extensions_get_extension_by_filepath() {
        let valid_mp3 = PathBuf::from("song.mp3");
        let valid_flac = PathBuf::from("track.flac");
        let unknown_file = PathBuf::from("document.txt");

        assert_eq!(AudioExtensions::get_extension_by_filepath(&valid_mp3), AudioExtensions::MP3);
        assert_eq!(AudioExtensions::get_extension_by_filepath(&valid_flac), AudioExtensions::FLAC);
        assert_eq!(AudioExtensions::get_extension_by_filepath(&unknown_file), AudioExtensions::UNKNOWN);
    }

    #[test]
    fn test_song_new_valid_extension() {
        let song = Song::new("test.mp3");
        assert!(song.is_some());
        let song = song.unwrap();
        assert_eq!(song.filename, "test.mp3");
        assert_eq!(song.extension, AudioExtensions::MP3);
        assert_eq!(song.filepath.to_str().unwrap(), "test.mp3");
        assert_eq!(false, song.is_file());
    }

    #[test]
    fn test_song_new_invalid_extension() {
        let song = Song::new("invalid_file.txt");
        assert!(song.is_none());
    }

    #[test]
    fn test_song_display() {
        let song = Song {
            filename: String::from("song.ogg"),
            extension: AudioExtensions::OGG,
            filepath: PathBuf::from("path/to/song.ogg"),
        };

        let output = format!("{}", song);
        assert!(output.contains("filename: song.ogg"));
        assert!(output.contains("extension: ogg"));
        assert!(output.contains("filepath: path/to/song.ogg"));
    }

    #[test]
    fn test_audio_extension_display() {
        assert_eq!(format!("{}", AudioExtensions::MP3), "mp3");
        assert_eq!(format!("{}", AudioExtensions::UNKNOWN), "unknown");
    }
}
