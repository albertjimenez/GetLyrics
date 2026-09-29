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
#[cfg(test)]
mod parallel_helper_tests {
    use super::ParallelHelper;
    use std::fs::{self, File};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn touch(path: &PathBuf) {
        File::create(path).unwrap();
    }

    #[test]
    fn collect_audio_files_non_recursive_returns_only_top_level_mp3_and_flac() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        let top_level_mp3 = root.join("song.mp3");
        let top_level_flac = root.join("album.flac");
        let top_level_txt = root.join("notes.txt");

        touch(&top_level_mp3);
        touch(&top_level_flac);
        touch(&top_level_txt);

        let nested_dir = root.join("nested");
        fs::create_dir(&nested_dir).unwrap();

        let nested_mp3 = nested_dir.join("nested-song.mp3");
        touch(&nested_mp3);

        let mut files = ParallelHelper::collect_audio_files(root, false);
        files.sort();

        assert_eq!(vec![top_level_flac, top_level_mp3], files);
    }

    #[test]
    fn collect_audio_files_recursive_returns_nested_mp3_and_flac_files() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        let top_level_mp3 = root.join("song.mp3");
        let top_level_flac = root.join("album.flac");
        let top_level_wav = root.join("ignored.wav");

        touch(&top_level_mp3);
        touch(&top_level_flac);
        touch(&top_level_wav);

        let nested_dir = root.join("nested");
        fs::create_dir(&nested_dir).unwrap();

        let nested_mp3 = nested_dir.join("nested-song.mp3");
        let nested_flac = nested_dir.join("nested-album.flac");
        let nested_txt = nested_dir.join("ignored.txt");

        touch(&nested_mp3);
        touch(&nested_flac);
        touch(&nested_txt);

        let mut files = ParallelHelper::collect_audio_files(root, true);
        files.sort();
let mut expected = vec![nested_flac, nested_mp3, top_level_flac, top_level_mp3];
        expected.sort();
        files.sort();
        assert_eq!(
            expected,
            files
        );
    }

    #[test]
    fn collect_audio_files_is_case_insensitive_for_supported_extensions() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        let upper_mp3 = root.join("song.MP3");
        let mixed_flac = root.join("album.FlAc");

        touch(&upper_mp3);
        touch(&mixed_flac);

        let mut files = ParallelHelper::collect_audio_files(root, false);
        files.sort();

        assert_eq!(vec![mixed_flac, upper_mp3], files);
    }

    #[test]
    fn collect_audio_files_returns_empty_vec_when_no_supported_files_exist() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        touch(&root.join("notes.txt"));
        touch(&root.join("audio.wav"));
        touch(&root.join("cover.jpg"));

        let files = ParallelHelper::collect_audio_files(root, false);

        assert!(files.is_empty());
    }
}