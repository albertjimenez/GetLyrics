use std::fs::File;

use audiotags::Tag;
use log::{error, warn};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
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
        if title == "" && artist == "" {
            warn!("Artist or title is empty. Skipping song.");
            return None;
        }
        let duration = Self::get_duration(borrowed_song2).map_or(None, |seconds| Some(seconds));
        return Some(SongMetadata { song: song.to_owned(), artist, title, album_title, duration });
    }
    fn get_duration(song: Song) -> Result<u16, Error> {
        let file = File::open(song.filepath)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let probed = get_probe().format(
            &Default::default(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let format = probed.format;
        let track = format
            .tracks()
            .first()
            .ok_or(Error::DecodeError("No audio tracks found"))?;

        let n_frames = track
            .codec_params
            .n_frames
            .ok_or(Error::DecodeError("Frame count missing"))?;

        let sample_rate = track
            .codec_params
            .sample_rate
            .ok_or(Error::DecodeError("Sample rate missing"))?;

        let seconds = (n_frames / sample_rate as u64) as u16;
        Ok(seconds)
    }
}
