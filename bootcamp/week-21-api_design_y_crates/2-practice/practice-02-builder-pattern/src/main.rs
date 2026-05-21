//! Patrón Builder con typestate — garantiza en tiempo de compilación
//! que los campos obligatorios han sido especificados.

use std::marker::PhantomData;

// Estados typestate
pub struct NoUrl;
pub struct WithUrl(String);

/// Builder para `HttpRequest` con typestate.
///
/// Solo se puede llamar a `send()` cuando se ha especificado una URL.
pub struct HttpRequestBuilder<U> {
    url: U,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    _marker: PhantomData<U>,
}

impl HttpRequestBuilder<NoUrl> {
    pub fn new() -> Self {
        HttpRequestBuilder {
            url: NoUrl,
            method: "GET".to_string(),
            headers: Vec::new(),
            body: None,
            _marker: PhantomData,
        }
    }

    pub fn url(self, url: impl Into<String>) -> HttpRequestBuilder<WithUrl> {
        HttpRequestBuilder {
            url: WithUrl(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            _marker: PhantomData,
        }
    }
}

impl Default for HttpRequestBuilder<NoUrl> {
    fn default() -> Self {
        Self::new()
    }
}

impl<U> HttpRequestBuilder<U> {
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    pub fn header(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.headers.push((key.into(), val.into()));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl HttpRequestBuilder<WithUrl> {
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url.0,
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
}

fn main() {
    let req = HttpRequestBuilder::new()
        .url("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(r#"{"name":"Ana"}"#)
        .build();

    println!("{} {}", req.method, req.url);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_request() {
        let r = HttpRequestBuilder::new()
            .url("https://example.com")
            .build();
        assert_eq!(r.method, "GET");
        assert_eq!(r.url, "https://example.com");
    }

    #[test]
    fn post_con_body() {
        let r = HttpRequestBuilder::new()
            .url("https://api.com")
            .method("POST")
            .body("data")
            .build();
        assert_eq!(r.body.as_deref(), Some("data"));
    }

    #[test]
    fn multiples_headers() {
        let r = HttpRequestBuilder::new()
            .url("https://a.com")
            .header("Accept", "application/json")
            .header("Authorization", "Bearer token")
            .build();
        assert_eq!(r.headers.len(), 2);
    }
}
