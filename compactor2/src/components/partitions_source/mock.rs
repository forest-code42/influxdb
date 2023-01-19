use std::fmt::Display;

use async_trait::async_trait;
use data_types::PartitionId;

use super::PartitionsSource;

#[derive(Debug)]
pub struct MockPartitionsSource {
    partitions: Vec<PartitionId>,
}

impl MockPartitionsSource {
    #[allow(dead_code)] // not used anywhere
    pub fn new(partitions: Vec<PartitionId>) -> Self {
        Self { partitions }
    }
}

impl Display for MockPartitionsSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mock")
    }
}

#[async_trait]
impl PartitionsSource for MockPartitionsSource {
    async fn fetch(&self) -> Vec<PartitionId> {
        self.partitions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(MockPartitionsSource::new(vec![]).to_string(), "mock",);
    }

    #[tokio::test]
    async fn test_fetch_empty() {
        assert_eq!(MockPartitionsSource::new(vec![]).fetch().await, vec![],);
    }

    #[tokio::test]
    async fn test_fetch_some() {
        let ids = vec![
            PartitionId::new(5),
            PartitionId::new(1),
            PartitionId::new(12),
        ];
        assert_eq!(MockPartitionsSource::new(ids.clone()).fetch().await, ids,);
    }
}
