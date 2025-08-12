//! Client

use bitcoin::Address;
use reqwest::{Client, Response};
use url::Url;

use crate::error::Error;
use crate::response::UserStats;

/// CKPool client
#[derive(Debug, Clone)]
pub struct CKPoolClient {
    url: Url,
    client: Client,
}

impl CKPoolClient {
    /// Construct a new CKPool client instance.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ckpool_api::prelude::*;
    ///
    /// let url: Url = Url::parse("https://solo.braiins.com").unwrap();
    /// let client = CKPoolClient::new(url);
    /// # let _client = client;
    /// ```
    #[inline]
    pub fn new(url: Url) -> Self {
        Self::from_client(url, Client::new())
    }

    /// Construct new with a custom reqwest [`Client`].
    #[inline]
    pub fn from_client(url: Url, client: Client) -> Self {
        Self { client, url }
    }

    /// Get user stats.
    pub async fn user_stats(&self, address: &Address) -> Result<UserStats, Error> {
        let url: Url = self
            .url
            .join("/users/")?
            .join(address.to_string().as_str())?;
        let response: Response = self.client.get(url).send().await?;

        match response.error_for_status() {
            Ok(res) => Ok(res.json().await?),
            Err(err) => match err.status() {
                Some(reqwest::StatusCode::NOT_FOUND) => Err(Error::UserNotFound),
                _ => Err(Error::from(err)),
            },
        }
    }
}
