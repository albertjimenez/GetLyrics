use std::env;
use std::fs;
use std::path::Path;

use env_logger::{Builder, Env};
use log::{error, info};

use GetLyrics::api::lrclib_api::LrcLibAPI;
use GetLyrics::api::lyric_api::LyricApi;
use GetLyrics::hasher::file_hash_helper::FileHashHelper;
use GetLyrics::metadata::metadata_extractor::MetadataExtractor;
use GetLyrics::model::data_model::{Lyric, Song, Writer};
use GetLyrics::traits::traits::LyricIface;

fn main() {
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: GetLyrics [-r|--recursive] [-k|--karaoke] <file_or_folder>");
    }
    let mut hasher = FileHashHelper::new().expect("Failed to init hashing");


    let mut karaoke = false;
    let mut recursive = false;
    let mut path: Option<String> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-k" | "--karaoke" => karaoke = true,
            "-r" | "--recursive" => recursive = true,
            _ => path = Some(arg.clone()),
        }
    }

    let Some(path) = path else {
        panic!("Usage: GetLyrics [-r|--recursive] [-k|--karaoke] <file_or_folder>");
    };

    let path_obj = Path::new(&path);

    if path_obj.is_dir() {
        process_directory(path_obj, karaoke, recursive, &mut hasher);
    } else if path_obj.is_file() {
        process_single_file(path_obj, karaoke, &mut hasher);
    } else {
        panic!("Invalid path: {}", path);
    }
}

fn process_directory(dir: &Path, karaoke: bool, recursive: bool, hasher: &mut FileHashHelper) {
    info!("Scanning directory: {}", dir.display());

    let audio_exts = ["mp3", "flac"];

    let entries = fs::read_dir(dir).expect("Cannot read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if recursive {
                    process_directory(&path, karaoke, recursive, hasher);
                }
                continue;
            }

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if audio_exts.contains(&ext.to_lowercase().as_str()) {
                    process_single_file(&path, karaoke, hasher);
                }
            }
        }
    }
}

fn process_single_file(path: &Path, karaoke: bool, hasher: &mut FileHashHelper) {


    // --- NEW: skip if already processed ---
    match hasher.should_process(path) {
        Ok(false) => {
            info!("Skipping already processed file: {}", path.display());
            return;
        }
        Err(e) => {
            error!("Hashing error for {}: {}", path.display(), e);
            return;
        }
        _ => {} // Ok(true) → continue
    }
    let file_path = path.to_str().unwrap();

    let song = match Song::new(file_path) {
        Some(song) => song,
        None => {
            error!("Could not create Song from {}", file_path);
            return;
        }
    };

    info!("Processing:\n\t {}", &song.filename);

    match MetadataExtractor::extract(&song) {
        Some(md) => {
            let lrclib_api = if karaoke {
                LrcLibAPI::new_karaoke_lyrics()
            } else {
                LrcLibAPI::new_plain_lyrics()
            };

            match lrclib_api.fetch_lyrics(&md) {
                Ok(lyric) => write_lyric_to_file(&song, &lyric),
                Err(_) => {
                    let secondary = LyricApi::new().fetch_lyrics(&md);
                    match secondary {
                        Ok(lyric) => write_lyric_to_file(&song, &lyric),
                        Err(e) => error!("{}", e),
                    }
                }
            }
        }
        None => error!("Could not extract metadata for {}", &song.filename),
    }
}

fn write_lyric_to_file(song: &Song, lyric: &Lyric) {
    match Writer::write_lyric(&lyric) {
        Some(_) => info!("SUCCESS: Lyrics written for: {}", &song.filename),
        None => error!("Could not write lyrics file for song {}", &song.filename),
    }
}
