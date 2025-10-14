use std::sync::LazyLock;

pub mod search;
pub mod types;

mod send;

static DEFAULT_HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(Default::default);

#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    /// Optionally use a specific client.
    http_client: Option<reqwest::Client>,
    /// Optionally specify a bearer token
    /// to access the search api.
    api_token: Option<String>,
}

impl Client {
    pub fn new(url: impl Into<String>) -> Client {
        Client {
            url: url.into(),
            http_client: None,
            api_token: None,
        }
    }

    pub fn with_client(mut self, http_client: reqwest::Client) -> Client {
        self.http_client = Some(http_client);
        self
    }

    pub fn with_api_token(mut self, api_token: impl Into<String>) -> Client {
        self.api_token = Some(api_token.into());
        self
    }

    fn http_client(&self) -> &reqwest::Client {
        self.http_client.as_ref().unwrap_or(&DEFAULT_HTTP_CLIENT)
    }
}
