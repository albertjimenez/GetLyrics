#[cfg(test)]
mod metadata_extractor_tests {
    use std::env;
    use crate::metadata::metadata_extractor::MetadataExtractor;
    use crate::model::data_model::Song;

    #[test]
    fn test_metadata() {
        let current_dir = env::current_dir().unwrap();
        let extensions = vec!["mp3"];
        for ext in extensions {
            let binding = current_dir.join(format!("test_resources/benny_blanco-roses.{}", ext));
            assert!(binding.is_file());
            let string_path = binding.to_str().unwrap();
            let song = Song::new(string_path);
            assert!(song.is_some());
            let song = song.unwrap();
            assert_eq!(true, song.is_file(), "File {} does not exist", &string_path);
            let metadata = MetadataExtractor::extract(&song);
            assert!(metadata.is_some());
            let metadata = metadata.unwrap();
            assert_eq!("Benny Blanco", metadata.artist);
            assert_eq!("Roses", metadata.title);
        }

    }
}