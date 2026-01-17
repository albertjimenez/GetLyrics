use crate::traits::traits::ProcessPolicy;
use std::path::Path;

pub struct DummyHasher;
impl DummyHasher {
    pub fn new() -> Box<dyn ProcessPolicy> {
        Box::new(DummyHasher)
    }
}
impl ProcessPolicy for DummyHasher {
    fn should_process(&self, _: &Path) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}
#[cfg(test)]
mod test_dummy_hasher {
    use crate::traits::traits::ProcessPolicy;
    use std::path::Path;
    use anyhow::Result;
    use crate::hasher::dummy_hasher::DummyHasher;

    #[test]
    fn dummy_hasher_always_processes() -> Result<()> {
        let hasher: Box<dyn ProcessPolicy> = DummyHasher::new();

        let fake_path = Path::new("does/not/matter.mp3");

        assert!(hasher.should_process(fake_path)?);
        assert!(hasher.should_process(fake_path)?);
        assert!(hasher.should_process(fake_path)?);

        Ok(())
    }
}
