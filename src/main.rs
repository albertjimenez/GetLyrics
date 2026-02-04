use rayon::iter::ParallelIterator;
use std::env;
use std::path::Path;
use std::process::exit;
use std::sync::Arc;
use env_logger::{Builder, Env};
use log::{debug, error, info};
use rayon::prelude::IntoParallelRefIterator;
use rayon::ThreadPoolBuilder;
use GetLyrics::api::lrclib_api::LrcLibAPI;
use GetLyrics::api::lyric_api::LyricApi;
use GetLyrics::hasher::dummy_hasher::DummyHasher;
use GetLyrics::hasher::file_hash_helper::FileHashHelper;
use GetLyrics::metadata::metadata_extractor::MetadataExtractor;
use GetLyrics::model::data_model::{Lyric, Song, Writer};
use GetLyrics::parallel_helper::parallel_helper::ParallelHelper;
use GetLyrics::traits::traits::{LyricIface, ProcessPolicy};

fn main() {
    // limit API pressure requests
    ThreadPoolBuilder::new().num_threads(6).build_global().unwrap();


    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Usage: GetLyrics [-r|--recursive] [-k|--karaoke] [-f|--force] <file_or_folder>");
        exit(1);
    }
    let mut hasher: Arc<dyn ProcessPolicy> = FileHashHelper::new_with_trait().expect("Failed to create file hasher");


    let mut karaoke = false;
    let mut recursive = false;
    let mut force_scan = false;
    let mut path: Option<String> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-k" | "--karaoke" => karaoke = true,
            "-r" | "--recursive" => recursive = true,
            "-f" | "--force" => force_scan = true,
            _ => path = Some(arg.clone()),
        }
    }

    let Some(path) = path else {
        panic!("Usage: GetLyrics [-r|--recursive] [-k|--karaoke] [-f|--force] <file_or_folder>");
    };

    let path_obj = Path::new(&path);

    if force_scan {
        info!("Running force scan.");
        hasher = DummyHasher::new();
    }

    if path_obj.is_dir() {
        process_directory(path_obj, karaoke, recursive, hasher);
    } else if path_obj.is_file() {
        process_single_file(path_obj, karaoke, hasher);
    } else {
        panic!("Invalid path: {}", path);
    }
}

fn process_directory(dir: &Path, karaoke: bool, recursive: bool, hasher: Arc<dyn ProcessPolicy>) {
    info!("Scanning directory: {}", dir.display());

    let files = ParallelHelper::collect_audio_files(dir, recursive);

    files.par_iter().for_each(|path| {
        process_single_file(path, karaoke, hasher.clone());
    });
}

fn process_single_file(path: &Path, karaoke: bool, hasher: Arc<dyn ProcessPolicy>) {


    // --- NEW: skip if already processed ---
    match hasher.should_process(path) {
        Ok(false) => {
            debug!("Skipping already processed file: {}", path.display());
            return;
        }
        Err(e) => {
            error!("Hashing error for {}: {}", path.display(), e);
            return;
        }
        Ok(true) => {}
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
