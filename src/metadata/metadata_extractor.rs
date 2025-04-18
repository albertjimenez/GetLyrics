use audiotags::Tag;
use log::{error, warn};
use crate::model::data_model::{Song, SongMetadata};

#[derive(Debug)]
pub struct MetadataExtractor {}

impl MetadataExtractor {
    pub fn extract(song: &Song) -> Option<SongMetadata> {
        let borrowed_song = song.to_owned();
        let tag = Tag::new().read_from_path(borrowed_song.filepath);
        if tag.is_err() {
            error!("Empty or invalid tags for {}", &borrowed_song.filename);
            return None;
        }
        let tag = tag.unwrap();
        let title = String::from(tag.title().unwrap_or(""));
        let artist = String::from(tag.artist().unwrap_or(""));
        if title == "" && artist == "" {
            warn!("Artist or title is empty. Skipping song.");
            return None;
        }
        return Some(SongMetadata { song: song.to_owned(), artist, title });
    }
}
