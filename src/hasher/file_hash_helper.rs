use crate::traits::traits::ProcessPolicy;
use anyhow::{Context, Result};
use dirs::home_dir;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, BufRead, Write},
    path::{Path},
};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct FileHashHelper {
    hashes: Mutex<HashSet<String>>,
    store_file: Mutex<File>, // adding a mutex to handle rayon parallelism
}

impl FileHashHelper {

    const FILENAME: &'static str = "processed_hashes.txt";
    pub fn new() -> Result<Self> {
        let mut dir = home_dir().context("Could not resolve home directory")?;
        dir.push(".getlyrics");

        if !dir.exists() {
            fs::create_dir_all(&dir).context("Failed to create ~/.getlyrics directory")?;
        }

        let mut store_path = dir.clone();
        store_path.push(FileHashHelper::FILENAME);

        if !store_path.exists() {
            fs::File::create(&store_path).context("Failed to create hash store file")?;
        }
        let file = File::open(&store_path)?;
        let reader = BufReader::new(file);
        let mut set = HashSet::new();

        for line in reader.lines().flatten() {
            set.insert(line);
        }

        // 🔹 Open file in append mode for future writes
        let store_file = OpenOptions::new()
            .append(true)
            .open(&store_path)?;

        Ok(Self {
            hashes: Mutex::new(set),
            store_file: Mutex::new(store_file),
        })
    }

    pub fn new_with_trait() -> Result<Arc<dyn ProcessPolicy>> {
        Ok(Arc::new(Self::new()?))
    }

    /// Compute SHA-256 of any file.
    fn hash_file(&self, path: &Path) -> Result<String> {
        let mut file =
            fs::File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;

        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = format!("{:x}", hasher.finalize());

        Ok(hash)
    }
}

impl ProcessPolicy for FileHashHelper {
    /// High-level helper: check + optionally store.
    fn should_process(&self, path: &Path) -> Result<bool> {
        let hash = self.hash_file(path)?;
        let mut set = self.hashes.lock().unwrap();
        if set.contains(&hash) {
            return Ok(false);
        }
        set.insert(hash.clone());
        drop(set); // release lock ASAP

        // Append to file (separate lock)
        let mut file = self.store_file.lock().unwrap();
        writeln!(file, "{}", hash)?;

        Ok(true)
    }
}


#[cfg(test)]
impl FileHashHelper {
    pub fn new_with_path(store_path: std::path::PathBuf) -> anyhow::Result<Self> {
        use std::collections::HashSet;
        use std::fs::{File, OpenOptions};
        use std::io::{BufRead, BufReader};
        use std::sync::Mutex;

        if !store_path.exists() {
            File::create(&store_path)?;
        }

        let reader = BufReader::new(File::open(&store_path)?);
        let mut set = HashSet::new();
        for line in reader.lines().flatten() {
            set.insert(line);
        }

        let file = OpenOptions::new().append(true).open(&store_path)?;

        Ok(Self {
            hashes: Mutex::new(set),
            store_file: Mutex::new(file),
        })
    }
}

#[cfg(test)]
mod test_file_hash_helper {
    use super::*;
    use crate::hasher::dummy_hasher::DummyHasher;
    use crate::traits::traits::ProcessPolicy;
    use anyhow::Result;
    use std::path::PathBuf;
    use std::sync::Arc;

    const DEFAULT_SONG_NAME: &str = "test_resources/benny_blanco-roses.mp3";

    fn make_test_helper(tmp_dir: &Path) -> Result<FileHashHelper> {
        let mut store = tmp_dir.to_path_buf();
        store.push(FileHashHelper::FILENAME);
        FileHashHelper::new_with_path(store)
    }

    #[test]
    fn test_hash_file_mp3() -> Result<()> {
        let mp3_path = PathBuf::from(DEFAULT_SONG_NAME);
        assert!(mp3_path.exists());

        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        let hash = helper.hash_file(&mp3_path)?;
        assert_eq!(hash.len(), 64);

        Ok(())
    }

    #[test]
    fn test_should_process_logic() -> Result<()> {
        let mp3_path = PathBuf::from(DEFAULT_SONG_NAME);
        let tmp_dir = tempfile::tempdir()?;
        let helper = make_test_helper(tmp_dir.path())?;

        assert!(helper.should_process(&mp3_path)?);
        assert!(!helper.should_process(&mp3_path)?);

        Ok(())
    }

    #[test]
    fn test_persistence_between_instances() -> Result<()> {
        let mp3_path = PathBuf::from(DEFAULT_SONG_NAME);
        let tmp_dir = tempfile::tempdir()?;

        // First run → store hash
        {
            let helper = make_test_helper(tmp_dir.path())?;
            assert!(helper.should_process(&mp3_path)?);
        }

        // Second instance should reload hashes from file
        {
            let helper = make_test_helper(tmp_dir.path())?;
            assert!(!helper.should_process(&mp3_path)?);
        }

        Ok(())
    }

    #[test]
    fn test_parallel_safety() -> Result<()> {
        use rayon::prelude::*;

        let mp3_path = PathBuf::from(DEFAULT_SONG_NAME);
        let tmp_dir = tempfile::tempdir()?;
        let helper = Arc::new(make_test_helper(tmp_dir.path())?);

        let results: Vec<bool> = (0..10)
            .into_par_iter()
            .map(|_| helper.should_process(&mp3_path).unwrap())
            .collect();

        // Only one thread should get true
        assert_eq!(results.iter().filter(|&&r| r).count(), 1);

        Ok(())
    }

    #[test]
    fn force_policy_overrides_real_policy() -> Result<()> {
        let mp3_path = PathBuf::from(DEFAULT_SONG_NAME);
        let tmp_dir = tempfile::tempdir()?;

        let real: Arc<dyn ProcessPolicy> = Arc::new(make_test_helper(tmp_dir.path())?);

        assert!(real.should_process(&mp3_path)?);
        assert!(!real.should_process(&mp3_path)?);

        let force: Arc<dyn ProcessPolicy> = DummyHasher::new();

        assert!(force.should_process(&mp3_path)?);
        assert!(force.should_process(&mp3_path)?);

        Ok(())
    }
}

