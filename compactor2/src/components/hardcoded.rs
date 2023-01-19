//! Current hardcoded component setup.
//!
//! TODO: Make this a runtime-config.

use std::sync::Arc;

use crate::config::Config;

use super::{
    partitions_source::{
        catalog::CatalogPartitionsSource, logging::LoggingPartitionsSourceWrapper,
        randomize_order::RandomizeOrderPartitionsSourcesWrapper,
    },
    Components,
};

/// Get hardcoded components.
pub fn hardcoded_components(config: &Config) -> Arc<Components> {
    // TODO: partitions source: Implementing ID-based sharding / hash-partitioning so we can run multiple compactors in
    //       parallel. This should be a wrapper around the existing partions source.

    Arc::new(Components {
        partitions_source: Arc::new(LoggingPartitionsSourceWrapper::new(
            RandomizeOrderPartitionsSourcesWrapper::new(
                CatalogPartitionsSource::new(
                    config.backoff_config.clone(),
                    Arc::clone(&config.catalog),
                    config.partition_minute_threshold,
                    Arc::clone(&config.time_provider),
                ),
                1234,
            ),
        )),
        file_filters: vec![],
        partition_filters: vec![],
    })
}
