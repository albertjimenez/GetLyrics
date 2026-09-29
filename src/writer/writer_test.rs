#[cfg(test)]
mod tests {
    use crate::model::data_model::{AudioExtensions, Lyric, Song, Writer};
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn test_write_lyric_success() {
        // Setup temporary directory and test file path

        let current_dir = env::current_dir().unwrap();
        let filename = "benny_blanco-roses.mp3";
        let song_path = current_dir.join(format!("test_resources/{}", filename));

        // Create Song and Lyric instances
        let song = Song {
            filename: String::from(filename),
            extension: AudioExtensions::MP3,
            filepath: song_path.clone(),
        };

        let lyric_text = String::from("Hello world, this is a lyric.");
        let lyric = Lyric {
            lyric: lyric_text.clone(),
            song,
        };

        // Call the writer
        let result = Writer::write_lyric(&lyric);

        // Assert output path is correct
        assert!(result.is_some());
        let output_path = result.unwrap();
        assert!(output_path.exists());

        // Assert content matches
        let written_content = fs::read_to_string(output_path).unwrap();
        assert_eq!(written_content, lyric_text);
    }

    #[test]
    fn test_write_lyric_invalid_path() {
        let invalid_song = Song {
            filename: "bad.mp3".to_string(),
            extension: AudioExtensions::MP3,
            filepath: PathBuf::from("///nonexistent/bad.mp3"),
        };

        let lyric = Lyric {
            lyric: String::from("This won't be written."),
            song: invalid_song,
        };

        let result = Writer::write_lyric(&lyric);
        assert!(result.is_none());
    }

    #[test]
    fn test_lyric_path_for_audio_replaces_extension_with_lrc() {
        let audio = PathBuf::from("/music/song.mp3");
        assert_eq!(
            Writer::lyric_path_for_audio(&audio),
            PathBuf::from("/music/song.lrc")
        );

        let audio = PathBuf::from("/music/song.flac");
        assert_eq!(
            Writer::lyric_path_for_audio(&audio),
            PathBuf::from("/music/song.lrc")
        );
    }

    #[test]
    fn test_has_lyrics_reports_presence_next_to_song() {
        let dir = tempfile::tempdir().unwrap();
        let song = dir.path().join("song.mp3");
        fs::write(&song, "fake-audio").unwrap();

        // No .lrc yet -> missing.
        assert!(!Writer::has_lyrics(&song));

        // Drop an .lrc next to it -> found.
        fs::write(Writer::lyric_path_for_audio(&song), "[00:01.00] hi").unwrap();
        assert!(Writer::has_lyrics(&song));
    }

    #[test]
    fn test_missing_lyrics_keeps_only_songs_without_lrc() {
        let dir = tempfile::tempdir().unwrap();
        let with_lrc = dir.path().join("with.mp3");
        let without_lrc = dir.path().join("without.mp3");
        fs::write(&with_lrc, "fake-audio").unwrap();
        fs::write(&without_lrc, "fake-audio").unwrap();
        fs::write(Writer::lyric_path_for_audio(&with_lrc), "lyrics").unwrap();

        let missing = Writer::missing_lyrics(&[with_lrc, without_lrc.clone()]);
        assert_eq!(missing, vec![without_lrc]);
    }

    #[test]
    fn test_missing_lyrics_high_level_scan_of_directory() {
        use crate::parallel_helper::parallel_helper::ParallelHelper;

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("done.mp3"), "fake-audio").unwrap();
        fs::write(root.join("done.lrc"), "lyrics").unwrap();
        fs::write(root.join("todo.mp3"), "fake-audio").unwrap();
        fs::write(root.join("todo.flac"), "fake-audio").unwrap();
        fs::write(root.join("notes.txt"), "ignored").unwrap();

        let files = ParallelHelper::collect_audio_files(root, false);
        assert_eq!(files.len(), 3);

        let mut missing = Writer::missing_lyrics(&files);
        missing.sort();
        assert_eq!(missing, vec![root.join("todo.flac"), root.join("todo.mp3")]);
    }

    #[test]
    fn test_write_lyric_and_has_lyrics_agree_on_location() {
        let dir = tempfile::tempdir().unwrap();
        let song_path = dir.path().join("roundtrip.mp3");
        fs::write(&song_path, "fake-audio").unwrap();
        assert!(!Writer::has_lyrics(&song_path));

        let song = Song {
            filename: String::from("roundtrip.mp3"),
            extension: AudioExtensions::MP3,
            filepath: song_path.clone(),
        };
        let lyric = Lyric {
            lyric: String::from("lyrics"),
            song,
        };
        let written = Writer::write_lyric(&lyric).unwrap();
        assert_eq!(written, Writer::lyric_path_for_audio(&song_path));
        assert!(Writer::has_lyrics(&song_path));
    }
}
