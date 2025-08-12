//! Responses

use serde::{Deserialize, Serialize};

/// User stats
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UserStats {
    /// Hashrate (1m)
    pub hashrate1m: String,
    /// Hashrate (5m)
    pub hashrate5m: String,
    /// Hashrate (1hr)
    pub hashrate1hr: String,
    /// Hashrate (1d)
    pub hashrate1d: String,
    /// Hashrate (7d)
    pub hashrate7d: String,
    /// Last share timestamp
    #[serde(rename = "lastshare")]
    pub last_share: u64,
    /// Number of workers
    pub workers: usize,
    /// Total shares
    pub shares: usize,
    /// Best share
    #[serde(rename = "bestshare")]
    pub best_share: f64,
    /// Best ever share
    #[serde(rename = "bestever")]
    pub best_ever: usize,
    /// Authorised timestamp
    pub authorised: u64,
    /// Workers
    pub worker: Vec<WorkerStats>,
}

/// Worker stats
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WorkerStats {
    /// Worker name
    #[serde(rename = "workername")]
    pub worker_name: String,
    /// Hashrate (1m)
    pub hashrate1m: String,
    /// Hashrate (5m)
    pub hashrate5m: String,
    /// Hashrate (1hr)
    pub hashrate1hr: String,
    /// Hashrate (1d)
    pub hashrate1d: String,
    /// Hashrate (7d)
    pub hashrate7d: String,
    /// Last share timestamp
    #[serde(rename = "lastshare")]
    pub last_share: u64,
    /// Total shares
    pub shares: usize,
    /// Best share
    #[serde(rename = "bestshare")]
    pub best_share: f64,
    /// Best ever share
    #[serde(rename = "bestever")]
    pub best_ever: usize,
}
