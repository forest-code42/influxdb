use metric::{Attributes, DurationHistogram, Metric};

pub fn assert_histogram_metric_count(metrics: &metric::Registry, name: &'static str, n: u64) {
    let histogram = metrics
        .get_instrument::<Metric<DurationHistogram>>("catalog_op_duration")
        .expect("failed to read metric")
        .get_observer(&Attributes::from(&[("op", name), ("result", "success")]))
        .expect("failed to get observer")
        .fetch();

    let hit_count = histogram.buckets.iter().fold(0, |acc, v| acc + v.count);
    assert_eq!(hit_count, n);
}
