use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::error::{DkpError, DkpResult};

use super::types::{
    ConfirmPublishResponse, DownloadUrlResponse, PackVersionResponse, PublishRequest,
    PublishResponse, SearchResponse, VersionListResponse,
};

pub struct RegistryClient {
    base_url: String,
    token: Option<String>,
    http: reqwest::Client,
}

impl RegistryClient {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            token,
            http: reqwest::Client::builder()
                .user_agent(concat!("dkp/", env!("CARGO_PKG_VERSION")))
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api/v1{}", self.base_url, path)
    }

    fn auth_header(&self) -> Option<String> {
        self.token.as_ref().map(|t| format!("Bearer {t}"))
    }

    pub async fn resolve(&self, name: &str, version: &str) -> DkpResult<PackVersionResponse> {
        let encoded = urlencoding::encode(name);
        let url = self.url(&format!("/packages/{encoded}/{version}"));
        let mut req = self.http.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header(AUTHORIZATION, auth);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if resp.status() == 401 {
            return Err(DkpError::Registry(
                "authentication required for this pack".into(),
            ));
        }
        if resp.status() == 403 {
            return Err(DkpError::Registry(
                "not authorized to access this pack".into(),
            ));
        }
        if resp.status() == 404 {
            return Err(DkpError::Registry(format!(
                "pack {name}@{version} not found"
            )));
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "registry error {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }

    pub async fn list_versions(&self, name: &str) -> DkpResult<VersionListResponse> {
        let encoded = urlencoding::encode(name);
        let url = self.url(&format!("/packages/{encoded}/versions"));
        let mut req = self.http.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header(AUTHORIZATION, auth);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "registry error {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }

    pub async fn publish(&self, req: PublishRequest) -> DkpResult<PublishResponse> {
        let url = self.url("/publish");
        let auth = self
            .auth_header()
            .ok_or_else(|| DkpError::Registry("registry token required for publish".into()))?;
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "publish failed {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }

    pub async fn confirm_publish(
        &self,
        name: &str,
        version: &str,
    ) -> DkpResult<ConfirmPublishResponse> {
        let encoded = urlencoding::encode(name);
        let url = self.url(&format!("/packages/{encoded}/{version}/confirm"));
        let auth = self
            .auth_header()
            .ok_or_else(|| DkpError::Registry("registry token required for confirm".into()))?;
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "confirm failed {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }

    pub async fn get_download_url(
        &self,
        name: &str,
        version: &str,
    ) -> DkpResult<DownloadUrlResponse> {
        let encoded = urlencoding::encode(name);
        let url = self.url(&format!("/packages/{encoded}/{version}/download-url"));
        let mut req = self.http.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header(AUTHORIZATION, auth);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if resp.status() == 401 {
            return Err(DkpError::Registry("authentication required".into()));
        }
        if resp.status() == 403 {
            return Err(DkpError::Registry("not authorized".into()));
        }
        if resp.status() == 404 {
            return Err(DkpError::Registry(format!("{name}@{version} not found")));
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "registry error {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }

    pub async fn yank(&self, name: &str, version: &str, reason: &str) -> DkpResult<()> {
        let url = self.url("/yank");
        let auth = self
            .auth_header()
            .ok_or_else(|| DkpError::Registry("registry token required for yank".into()))?;
        let body = serde_json::json!({ "name": name, "version": version, "reason": reason });
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!("yank failed {status}: {body}")));
        }
        Ok(())
    }

    pub async fn deprecate(
        &self,
        name: &str,
        version: &str,
        deprecated: bool,
        message: Option<&str>,
    ) -> DkpResult<()> {
        let url = self.url("/deprecate");
        let auth = self
            .auth_header()
            .ok_or_else(|| DkpError::Registry("registry token required for deprecate".into()))?;
        let body = serde_json::json!({
            "name": name,
            "version": version,
            "deprecated": deprecated,
            "message": message,
        });
        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "deprecate failed {status}: {body}"
            )));
        }
        Ok(())
    }

    pub async fn search(
        &self,
        q: &str,
        domain: Option<&str>,
        conformance: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> DkpResult<SearchResponse> {
        let mut url = reqwest::Url::parse(&self.url("/search"))
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("q", q)
            .append_pair("limit", &limit.to_string())
            .append_pair("offset", &offset.to_string());
        if let Some(d) = domain {
            url.query_pairs_mut().append_pair("domain", d);
        }
        if let Some(c) = conformance {
            url.query_pairs_mut().append_pair("conformance", c);
        }
        let mut req = self.http.get(url);
        if let Some(auth) = self.auth_header() {
            req = req.header(AUTHORIZATION, auth);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DkpError::Registry(format!(
                "search failed {status}: {body}"
            )));
        }
        resp.json()
            .await
            .map_err(|e| DkpError::Registry(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn sample_manifest_json() -> serde_json::Value {
        serde_json::json!({
            "spec": "dkp/0.2",
            "name": "test-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "test",
            "known_limitations": "none",
            "update_date": "2026-01-01"
        })
    }

    fn sample_pack_version_response() -> serde_json::Value {
        serde_json::json!({
            "name": "test-pack",
            "version": "1.0.0",
            "manifest": sample_manifest_json(),
            "checksums": {},
            "bundle_sig": "sig",
            "archive_format": "tar.xz",
            "publisher_public_key": "pubkey",
            "published_at": "2026-01-01T00:00:00Z",
            "conformance": "dkp-conformant",
            "visibility": "public",
            "yanked": false,
            "yank_reason": null,
            "deprecated": false,
            "deprecation_message": null,
            "eval_summary": null,
            "readme": null,
            "download_count": 0
        })
    }

    #[tokio::test]
    async fn resolve_200_returns_pack_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/test-pack/1.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_pack_version_response()))
            .mount(&server)
            .await;

        let client = RegistryClient::new(server.uri(), None);
        let result = client.resolve("test-pack", "1.0.0").await.unwrap();
        assert_eq!(result.name, "test-pack");
        assert_eq!(result.version, "1.0.0");
    }

    #[tokio::test]
    async fn resolve_401_maps_to_authentication_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/test-pack/1.0.0"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = RegistryClient::new(server.uri(), None);
        let err = client.resolve("test-pack", "1.0.0").await.unwrap_err();
        assert!(matches!(err, DkpError::Registry(msg) if msg.contains("authentication required")));
    }

    #[tokio::test]
    async fn resolve_403_maps_to_authorization_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/test-pack/1.0.0"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = RegistryClient::new(server.uri(), None);
        let err = client.resolve("test-pack", "1.0.0").await.unwrap_err();
        assert!(matches!(err, DkpError::Registry(msg) if msg.contains("not authorized")));
    }

    #[tokio::test]
    async fn resolve_404_maps_to_not_found_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/test-pack/1.0.0"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = RegistryClient::new(server.uri(), None);
        let err = client.resolve("test-pack", "1.0.0").await.unwrap_err();
        assert!(matches!(err, DkpError::Registry(msg) if msg.contains("not found")));
    }
}
