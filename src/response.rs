//! Responses

use serde::{Deserialize, Deserializer};

/// User stats
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct UserStats {
    /// Hashrate (1m) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1m: f64,
    /// Hashrate (5m) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate5m: f64,
    /// Hashrate (1hr) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1hr: f64,
    /// Hashrate (1d) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1d: f64,
    /// Hashrate (7d) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate7d: f64,
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct WorkerStats {
    /// Worker name
    #[serde(rename = "workername")]
    pub worker_name: String,
    /// Hashrate (1m) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1m: f64,
    /// Hashrate (5m) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate5m: f64,
    /// Hashrate (1hr) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1hr: f64,
    /// Hashrate (1d) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate1d: f64,
    /// Hashrate (7d) in hash/s
    #[serde(deserialize_with = "parse_hashrate")]
    pub hashrate7d: f64,
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

/// Parse hashrate string (e.g., "6.2M", "386M", "0") into f64
fn parse_hashrate<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    if s == "0" {
        return Ok(0.0);
    }

    let s: &str = s.trim();

    if s.is_empty() {
        return Ok(0.0);
    }

    // Check if the last character is a unit suffix
    let (number_part, multiplier) = if let Some(last_char) = s.chars().last() {
        match last_char {
            'K' | 'k' => (&s[..s.len() - 1], 1_000.0),
            'M' | 'm' => (&s[..s.len() - 1], 1_000_000.0),
            'G' | 'g' => (&s[..s.len() - 1], 1_000_000_000.0),
            'T' | 't' => (&s[..s.len() - 1], 1_000_000_000_000.0),
            'P' | 'p' => (&s[..s.len() - 1], 1_000_000_000_000_000.0),
            'E' | 'e' => (&s[..s.len() - 1], 1_000_000_000_000_000_000.0),
            _ => (s, 1.0), // No suffix, treat as plain number
        }
    } else {
        (s, 1.0)
    };

    // Parse the numeric part
    let number: f64 = number_part.parse().map_err(serde::de::Error::custom)?;

    // Return the result
    Ok(number * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_stats_deserialization() {
        let json = r#"{
            "hashrate1m": "0",
            "hashrate5m": "1K",
            "hashrate1hr": "6.2M",
            "hashrate1d": "7G",
            "hashrate7d": "2T",
            "lastshare": 1747672247,
            "workers": 0,
            "shares": 323295,
            "bestshare": 105654.8966108053,
            "bestever": 105654,
            "authorised": 1740481437,
            "worker": [
                {
                    "workername": "bc1qz9vvexjmexe8pr2aueuz6x0v94ulkx2m2sp6lr",
                    "hashrate1m": "3P",
                    "hashrate5m": "4E",
                    "hashrate1hr": "0",
                    "hashrate1d": "6.19M",
                    "hashrate7d": "340M",
                    "lastshare": 1747672247,
                    "shares": 139320,
                    "bestshare": 49794.22067862848,
                    "bestever": 49794
                }
            ]
        }"#;

        let stats: UserStats = serde_json::from_str(json).unwrap();

        // Test main stats
        assert_eq!(stats.hashrate1m, 0.0);
        assert_eq!(stats.hashrate5m, 1_000.0);
        assert_eq!(stats.hashrate1hr, 6.2 * 10.0_f64.powf(6.0));
        assert_eq!(stats.hashrate1d, 7.0 * 10.0_f64.powf(9.0));
        assert_eq!(stats.hashrate7d, 2.0 * 10.0_f64.powf(12.0));
        assert_eq!(stats.last_share, 1747672247);
        assert_eq!(stats.workers, 0);
        assert_eq!(stats.shares, 323295);
        assert_eq!(stats.best_share, 105654.8966108053);
        assert_eq!(stats.best_ever, 105654);
        assert_eq!(stats.authorised, 1740481437);

        // Test worker stats
        assert_eq!(stats.worker.len(), 1);
        let worker = &stats.worker[0];
        assert_eq!(
            worker.worker_name,
            "bc1qz9vvexjmexe8pr2aueuz6x0v94ulkx2m2sp6lr"
        );
        assert_eq!(worker.hashrate1m, 3.0 * 10.0_f64.powf(15.0));
        assert_eq!(worker.hashrate5m, 4.0 * 10.0_f64.powf(18.0));
        assert_eq!(worker.hashrate1hr, 0.0);
        assert_eq!(worker.hashrate1d, 6_190_000.0);
        assert_eq!(worker.hashrate7d, 340_000_000.0);
        assert_eq!(worker.last_share, 1747672247);
        assert_eq!(worker.shares, 139320);
        assert_eq!(worker.best_share, 49794.22067862848);
        assert_eq!(worker.best_ever, 49794);
    }
}
