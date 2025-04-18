#[cfg(test)]
mod tests {
    use crate::model::data_model::{LyricScraperV1, Song, SongMetadata};
    use crate::traits::traits::LyricScraper;

    #[test]
    fn test_fetch_valid_lyrics() {
        let scraper = LyricScraperV1::new();

        let metadata = SongMetadata {
            artist: "Barbara Bandeira".to_string(),
            title: "Onde Vais".to_string(),
            song: Song::empty()
        };

        let result = scraper.fetch_lyrics(&metadata);
        assert!(result.is_ok(), "Expected lyrics to be found, got error: {:?}", result.err());

        let lyrics = result.unwrap();
        assert!(!lyrics.trim().is_empty(), "Lyrics were empty");
    }

    #[test]
    fn test_fetch_valid_lyrics_with_slight_misspelling() {
        let scraper = LyricScraperV1::new();

        let metadata = SongMetadata {
            artist: "Barbara Bandeira".to_string(),
            title: "Onde Vaiss".to_string(),
            song: Song::empty()
        };

        let result = scraper.fetch_lyrics(&metadata);
        assert!(result.is_ok(), "Expected lyrics to be found, got error: {:?}", result.err());

        let lyrics = result.unwrap();
        assert!(!lyrics.trim().is_empty(), "Lyrics were empty");
    }

    #[test]
    fn test_fetch_nonexistent_lyrics() {
        let scraper = LyricScraperV1::new();

        let metadata = SongMetadata {
            artist: "@#rwfuisdojokvdf 75sdf5This Artist Does Not Exist5ddfdff6df56df645".to_string(),
            title: "Fake sad75fSong Title That sdf75sdWill Fail5sdsdf86sdfsdfsdfsdf".to_string(),
            song: Song::empty()
        };

        let result = scraper.fetch_lyrics(&metadata);
        assert!(result.is_err(), "Expected failure for nonexistent lyrics, got {:?}", result.ok());
        println!("Error as expected: {}", result.err().unwrap());
    }
}
