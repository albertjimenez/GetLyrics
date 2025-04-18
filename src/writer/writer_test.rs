#[cfg(test)]
mod tests {
    use std::{env, fs};
    use std::path::PathBuf;
    use crate::model::data_model::{Lyric, Song, AudioExtensions, Writer};

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
}
