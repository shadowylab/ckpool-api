//! Client

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
    pub async fn user_stats(&self, user: &str) -> Result<UserStats, Error> {
        let url: Url = self.url.join("/users/")?.join(user)?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }
}
