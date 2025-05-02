use std::env;

use env_logger::{Builder, Env};
use log::{error, info};

use crate::api::lrclib_api::LrcLibAPI;
use crate::api::lyric_api::LyricApi;
use crate::metadata::metadata_extractor::MetadataExtractor;
use crate::model::data_model::{Lyric, Song, Writer};
use crate::traits::traits::LyricIface;

pub mod model;
mod api;
mod metadata;
mod writer;
mod traits;

fn main() {
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: GetLyrics [-k|--karaoke] /path/to/song.mp3");
    }
    let (karaoke, file_path) = if args.len() == 3 && (args[1] == "-k" || args[1] == "--karaoke") {
        (true, &args[2])
    } else if args.len() == 2 {
        (false, &args[1])
    } else {
        panic!("Usage: GetLyrics [-k|--karaoke] /path/to/song.mp3");
    };
    let song = Song::new(file_path).unwrap();
    info!("Processing:\n\t {}", &song.filename);
    match MetadataExtractor::extract(&song) {
        Some(md) => {
            let lrclib_api = if karaoke {
                LrcLibAPI::new_karaoke_lyrics()
            } else {
                LrcLibAPI::new_plain_lyrics()
            };

            match lrclib_api.fetch_lyrics(&md) {
                Ok(lyric) => {
                    write_lyric_to_file(&song, &lyric);
                }
                Err(_) => {
                    let secondary_lyric_api = LyricApi::new().fetch_lyrics(&md);
                    match secondary_lyric_api {
                        Ok(lyric) => {
                            write_lyric_to_file(&song, &lyric);
                        }
                        Err(e) => {
                            error!("{}", e)
                        }
                    }
                }
            }
        }
        None => {
            error!("Could not extract metadata for {}", &song.filename);
        }
    }
}

fn write_lyric_to_file(song: &Song, lyric: &Lyric) {
    match Writer::write_lyric(&lyric) {
        Some(path) => info!("SUCCESS: Lyrics written for: {}", &song.filename),
        None => error!("Could not write lyrics file for song {}", &song.filename),
    }
}

