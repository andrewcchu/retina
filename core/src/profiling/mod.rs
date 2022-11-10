use std::time::{Duration};

use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone)]
pub struct FeatureCosts {
    pub p_feature_process_mem: usize,
    pub p_feature_cycle_start: u64,
    pub p_feature_cycles: Duration,
    pub p_feature_storage: usize,
}

impl FeatureCosts {
    pub fn new(p_feature_process_mem: usize, p_feature_cycle_start: u64, p_feature_cycles: Duration, p_feature_storage: usize) -> Self {
        FeatureCosts {
            p_feature_process_mem: p_feature_process_mem,
            p_feature_cycle_start: p_feature_cycle_start,
            p_feature_cycles: p_feature_cycles,
            p_feature_storage: p_feature_storage,
        }
    }
}