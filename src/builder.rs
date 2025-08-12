//! Client builder

#[cfg(feature = "socks")]
use std::net::SocketAddr;
use std::time::Duration;

#[cfg(feature = "socks")]
use reqwest::Proxy;
use reqwest::{Client, ClientBuilder};
use url::Url;

use crate::client::CKPoolClient;
use crate::error::Error;

/// CKPool client builder
#[derive(Debug, Clone)]
pub struct CKPoolClientBuilder {
    /// Endpoint URL
    pub url: Url,
    /// Timeout for requests
    pub timeout: Duration,
    /// Socks5 proxy
    #[cfg(feature = "socks")]
    pub proxy: Option<SocketAddr>,
}

impl CKPoolClientBuilder {
    /// Construct a new builder
    pub fn new(url: Url) -> Self {
        Self {
            url,
            timeout: Duration::from_secs(60),
            #[cfg(feature = "socks")]
            proxy: None,
        }
    }

    /// Set a custom timeout
    #[inline]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set proxy
    #[inline]
    #[cfg(feature = "socks")]
    pub fn proxy(mut self, proxy: SocketAddr) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Build mempool client
    pub fn build(self) -> Result<CKPoolClient, Error> {
        // Construct builder
        let mut builder: ClientBuilder = Client::builder();

        // Set proxy
        #[cfg(all(feature = "socks", not(target_arch = "wasm32")))]
        if let Some(proxy) = self.proxy {
            let proxy: String = format!("socks5h://{proxy}");
            builder = builder.proxy(Proxy::all(proxy)?);
        }

        // Set timeout
        builder = builder.timeout(self.timeout);

        // Build client
        let client: Client = builder.build()?;

        // Construct client
        Ok(CKPoolClient::from_client(self.url, client))
    }
}
