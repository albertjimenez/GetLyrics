use std::env;

use env_logger::{Builder, Env};
use log::{error, info};

use crate::api::lyric_api::LyricApi;
use crate::metadata::metadata_extractor::MetadataExtractor;
use crate::model::data_model::{Lyric, LyricScraperV1, Song, Writer};
use crate::traits::traits::LyricScraper;

pub mod model;
mod api;
mod metadata;
mod writer;
mod scrapper;
mod traits;

fn main() {
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Invalid number of arguments. Usage: GetLyrics /filepath/of/the/song")
    }
    let file_path = &args[1];
    info!("Processing {}", file_path);
    let song = Song::new(file_path).unwrap();
    let md = MetadataExtractor::extract(&song);
    if md.is_some() {
        let md = md.unwrap();
        let mut lyric = LyricApi::get_lyrics(&md);
        if lyric.is_none() {
            let lyric_v1 = LyricScraperV1::new();
            let scrapped_lyric = lyric_v1.fetch_lyrics(&md);
            if scrapped_lyric.is_err() {
                error!("{}" ,scrapped_lyric.err().unwrap_or(String::from("Could not find the lyrics using the scrapper.")));
                return;
            }
            let clonned_song = song.clone();
            lyric = Some(Lyric { lyric: scrapped_lyric.unwrap(), song: clonned_song });



        }
        let output_file = Writer::write_lyric(&lyric.unwrap());
        if output_file.is_some() {
            info!("SUCCESS: Finished writing lyrics for song {}", &song.filename);
            return;
        } else {
            error!("Could not write lyrics file for song {}", &song.filename);
            return;
        }
    } else {
        error!("Could not extract metadata for {}", &song.filename)
    }
}

