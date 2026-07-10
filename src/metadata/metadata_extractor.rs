use std::fs::File;

use audiotags::Tag;
use log::{error, warn};
use symphonia::core::errors::Error;
use symphonia::core::errors::Error::DecodeError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::units::Timestamp;
use symphonia::default::get_probe;

use crate::model::data_model::{Song, SongMetadata};

#[derive(Debug)]
pub struct MetadataExtractor {}

impl MetadataExtractor {
    pub fn extract(song: &Song) -> Option<SongMetadata> {
        let borrowed_song = song.to_owned();
        let borrowed_song2 = song.to_owned();
        let tag = Tag::new().read_from_path(borrowed_song.filepath);
        if tag.is_err() {
            error!("Empty or invalid tags for {}", &borrowed_song.filename);
            return None;
        }
        let tag = tag.unwrap();
        let title = String::from(tag.title().unwrap_or(""));
        let artist = String::from(tag.artist().unwrap_or(""));
        let album_title = String::from(tag.album_title().unwrap_or(""));
        if title.is_empty() && artist.is_empty() {
            warn!("Artist or title is empty. Skipping song.");
            return None;
        }
        let duration = Self::get_duration(borrowed_song2).map_or(None, |seconds| Some(seconds));
        Some(SongMetadata { song: song.to_owned(), artist, title, album_title, duration })
    }
    fn get_duration(song: Song) -> Result<u16, Error> {
        let file = File::open(song.filepath)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let probed = get_probe().probe(
            &Default::default(),
            mss,
            FormatOptions::default(),
            MetadataOptions::default(),
        )?;
        let track = probed
            .tracks()
            .first()
            .ok_or(DecodeError("No audio tracks found"))?;
        
        let time_base = track
            .time_base
            .ok_or(DecodeError("Track timebase missing"))?;

        let seconds = track
            .duration
            .and_then(|dur| time_base.calc_time(Timestamp::new(dur.get() as i64)))
            .map(|time| time.as_secs() as u16)
            .ok_or(DecodeError("Could not get the seconds"))?;
        Ok(seconds)
    }
}
