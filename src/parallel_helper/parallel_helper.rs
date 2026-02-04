use crate::model::data_model::AudioExtensions;
use std::path::Path;
use walkdir::WalkDir;

pub struct ParallelHelper;

impl ParallelHelper {
    // traverse to collect all supported audio files
    pub fn collect_audio_files(dir: &Path, recursive: bool) -> Vec<std::path::PathBuf> {
        let audio_exts = [AudioExtensions::MP3, AudioExtensions::FLAC];

        let walker = if recursive {
            WalkDir::new(dir).into_iter()
        } else {
            WalkDir::new(dir).max_depth(1).into_iter()
        };

        walker
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|path| {
                audio_exts.contains(&AudioExtensions::get_extension_by_filepath(path))
            })
            .collect()
    }
}
