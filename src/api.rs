/// Calls `api.contextdrive.io`. Debug output redacts the bearer token.
pub struct Client {
    base_url: String,
    token: Option<String>,
    http: reqwest::blocking::Client,
}

impl Client {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Result<Self, reqwest::Error> {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        let http = reqwest::blocking::Client::builder().build()?;
        Ok(Self {
            base_url,
            token,
            http,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn authorization(&self) -> Option<String> {
        self.token.as_ref().map(|token| format!("Bearer {token}"))
    }

    pub fn http(&self) -> &reqwest::blocking::Client {
        &self.http
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_ref().map(|_| "redacted"))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_redacts_the_token() {
        let client = Client::new(
            "https://api.contextdrive.io/",
            Some("secret-token".to_string()),
        )
        .unwrap();
        let rendered = format!("{client:?}");
        assert!(!rendered.contains("secret-token"));
        assert!(rendered.contains("redacted"));
        assert_eq!(client.base_url(), "https://api.contextdrive.io");
        assert_eq!(
            client.authorization().as_deref(),
            Some("Bearer secret-token")
        );
        let _ = client.http();
    }
}
