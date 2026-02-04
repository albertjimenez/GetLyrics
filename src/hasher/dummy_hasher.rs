use crate::traits::traits::ProcessPolicy;
use std::path::Path;
use std::sync::Arc;

pub struct DummyHasher;
impl DummyHasher {
    pub fn new() -> Arc<dyn ProcessPolicy> {
        Arc::new(DummyHasher)
    }
}
impl ProcessPolicy for DummyHasher {
    fn should_process(&self, _: &Path) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}
#[cfg(test)]
mod test_dummy_hasher {
    use crate::hasher::dummy_hasher::DummyHasher;
    use crate::traits::traits::ProcessPolicy;
    use anyhow::Result;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    fn dummy_hasher_always_processes() -> Result<()> {
        let hasher: Arc<dyn ProcessPolicy> = DummyHasher::new();

        let fake_path = Path::new("does/not/matter.mp3");

        assert!(hasher.should_process(fake_path)?);
        assert!(hasher.should_process(fake_path)?);
        assert!(hasher.should_process(fake_path)?);

        Ok(())
    }
}
