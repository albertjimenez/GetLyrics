use anyhow::{Context, Result};
use dirs::home_dir;
use sha2::{Digest, Sha256};
use std::{fs, io::{self, BufRead, Write}, path::{Path, PathBuf}};

pub struct FileHashHelper {
    store_path: PathBuf,
}

impl FileHashHelper {
    pub fn new() -> Result<Self> {
        let mut dir = home_dir().context("Could not resolve home directory")?;
        dir.push(".getlyrics");

        if !dir.exists() {
            fs::create_dir_all(&dir)
                .context("Failed to create ~/.getlyrics directory")?;
        }

        let mut store_path = dir.clone();
        store_path.push("processed_hashes.txt");

        if !store_path.exists() {
            fs::File::create(&store_path)
                .context("Failed to create hash store file")?;
        }

        Ok(Self { store_path })
    }

    /// Compute SHA-256 of any file.
    pub fn hash_file(&self, path: &Path) -> Result<String> {
        let mut file = fs::File::open(path)
            .with_context(|| format!("Failed to open file: {:?}", path))?;

        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = format!("{:x}", hasher.finalize());

        Ok(hash)
    }

    /// Check if the hash exists in the store.
    pub fn has_hash(&self, hash: &str) -> Result<bool> {
        let file = fs::File::open(&self.store_path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(existing) = line {
                if existing == hash {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Append new hash to the store.
    pub fn store_hash(&self, hash: &str) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&self.store_path)?;

        writeln!(file, "{}", hash)?;
        Ok(())
    }

    /// High-level helper: check + optionally store.
    pub fn should_process(&self, path: &Path) -> Result<bool> {
        let hash = self.hash_file(path)?;

        if self.has_hash(&hash)? {
            Ok(false)
        } else {
            self.store_hash(&hash)?;
            Ok(true)
        }
    }
}


#[cfg(test)]
mod test_file_hash_helper {
    use crate::hasher::file_hash_helper::FileHashHelper;
    use anyhow::Result;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    /// Build a FileHashHelper with a temporary hash store path.
    /// This avoids interfering with the real ~/.getlyrics folder.
    fn make_test_helper(tmp_dir: &Path) -> Result<FileHashHelper> {
        let helper = FileHashHelper::new()?;

        // Override the internal store path for testing:
        let mut store = tmp_dir.to_path_buf();
        store.push("test_hashes.txt");

        fs::File::create(&store).expect("failed to create temporary test hash store");

        // Rebuild helper with the overridden path
        Ok(FileHashHelper {
            store_path: store,
            ..helper
        })
    }

    #[test]
    fn test_hash_file_mp3() -> Result<()> {
        // Path to the MP3 located outside `src/`:
        let mp3_path = PathBuf::from("test_resources/benny_blanco-roses.mp3");
        assert!(mp3_path.exists(), "MP3 file should exist in project root");

        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        let hash = helper.hash_file(&mp3_path)?;
        assert!(!hash.is_empty(), "Hash should not be empty");
        assert_eq!(hash.len(), 64, "SHA-256 should be 64 hex chars");

        Ok(())
    }

    #[test]
    fn test_store_and_check_hash() -> Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        let fake_hash = "abc123deadbeef";

        // Initially it should not exist
        assert!(!helper.has_hash(fake_hash)?, "Hash should not exist yet");

        // Store the hash
        helper.store_hash(fake_hash)?;

        // Now it must exist
        assert!(helper.has_hash(fake_hash)?, "Hash should exist after storing");

        Ok(())
    }

    #[test]
    fn test_should_process_logic() -> Result<()> {
        let mp3_path = PathBuf::from("test_resources/benny_blanco-roses.mp3");

        assert!(mp3_path.exists(), "MP3 must exist");

        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        // First call should process the file
        let first = helper.should_process(&mp3_path)?;
        assert!(first, "First call should return true — not processed yet");

        // Second call should skip the file
        let second = helper.should_process(&mp3_path)?;
        assert!(!second, "Second call should return false — already processed");

        Ok(())
    }

    #[test]
    fn test_store_file_created() -> Result<()> {
        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        // The helper should have created a test hash store file
        assert!(
            helper.store_path.exists(),
            "Hash store file should exist inside tmp_dir"
        );

        Ok(())
    }
}
