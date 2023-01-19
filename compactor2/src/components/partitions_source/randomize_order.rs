use std::fmt::Display;

use async_trait::async_trait;
use data_types::PartitionId;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use super::PartitionsSource;

#[derive(Debug)]
pub struct RandomizeOrderPartitionsSourcesWrapper<T>
where
    T: PartitionsSource,
{
    inner: T,
    seed: u64,
}

impl<T> RandomizeOrderPartitionsSourcesWrapper<T>
where
    T: PartitionsSource,
{
    pub fn new(inner: T, seed: u64) -> Self {
        Self { inner, seed }
    }
}

impl<T> Display for RandomizeOrderPartitionsSourcesWrapper<T>
where
    T: PartitionsSource,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "randomize_order({})", self.inner)
    }
}

#[async_trait]
impl<T> PartitionsSource for RandomizeOrderPartitionsSourcesWrapper<T>
where
    T: PartitionsSource,
{
    async fn fetch(&self) -> Vec<PartitionId> {
        let mut partitions = self.inner.fetch().await;
        let mut rng = StdRng::seed_from_u64(self.seed);
        partitions.shuffle(&mut rng);
        partitions
    }
}

#[cfg(test)]
mod tests {
    use crate::components::partitions_source::mock::MockPartitionsSource;

    use super::*;

    #[test]
    fn test_display() {
        let source =
            RandomizeOrderPartitionsSourcesWrapper::new(MockPartitionsSource::new(vec![]), 123);
        assert_eq!(source.to_string(), "randomize_order(mock)",);
    }

    #[tokio::test]
    async fn test_fetch_empty() {
        let source =
            RandomizeOrderPartitionsSourcesWrapper::new(MockPartitionsSource::new(vec![]), 123);
        assert_eq!(source.fetch().await, vec![],);
    }

    #[tokio::test]
    async fn test_fetch_some() {
        let ids = vec![
            PartitionId::new(5),
            PartitionId::new(1),
            PartitionId::new(12),
        ];

        // shuffles
        let source = RandomizeOrderPartitionsSourcesWrapper::new(
            MockPartitionsSource::new(ids.clone()),
            123,
        );
        assert_eq!(
            source.fetch().await,
            vec![
                PartitionId::new(12),
                PartitionId::new(1),
                PartitionId::new(5),
            ],
        );

        // is deterministic in same source
        for _ in 0..100 {
            assert_eq!(
                source.fetch().await,
                vec![
                    PartitionId::new(12),
                    PartitionId::new(1),
                    PartitionId::new(5),
                ],
            );
        }

        // is deterministic with new source
        for _ in 0..100 {
            let source = RandomizeOrderPartitionsSourcesWrapper::new(
                MockPartitionsSource::new(ids.clone()),
                123,
            );
            assert_eq!(
                source.fetch().await,
                vec![
                    PartitionId::new(12),
                    PartitionId::new(1),
                    PartitionId::new(5),
                ],
            );
        }

        // different seed => different output
        let source = RandomizeOrderPartitionsSourcesWrapper::new(
            MockPartitionsSource::new(ids.clone()),
            1234,
        );
        assert_eq!(
            source.fetch().await,
            vec![
                PartitionId::new(1),
                PartitionId::new(12),
                PartitionId::new(5),
            ],
        );
    }
}
