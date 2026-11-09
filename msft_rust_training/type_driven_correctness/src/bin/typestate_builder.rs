mod request {
    use core::time;
    use std::time::Duration;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Method {
        Get,
        Post,
    }

    /// Marker type: "this required field has not been set yet".
    pub struct Missing;

    /// Fields are private, so the only way to get a `Request` is `builder()...build()`
    #[derive(Debug)]
    pub struct Request {
        url: String,
        method: Method,
        timeout: Duration,
        headers: Vec<(String, String)>,
    }

    impl Request {
        pub fn builder() -> RequestBuilder<Missing, Missing> {
            RequestBuilder {
                url: Missing,
                method: Missing,
                timeout: None,
                headers: Vec::new(),
            }
        }

        pub fn url(&self) -> &str {
            &self.url
        }
        pub fn method(&self) -> Method {
            self.method
        }
        pub fn timeout(&self) -> Duration {
            self.timeout
        }
        pub fn headers(&self) -> &[(String, String)] {
            &self.headers
        }
    }

    /// `Url` is `Missing` or `String`; `Meth` is `Missing` or `Method`
    /// The type parameter itself stores the value, so no `Option` + `unwrap()` is needed
    #[must_use = "a builder does nothing until `.build()` is called"]
    pub struct RequestBuilder<Url, Meth> {
        url: Url,
        method: Meth,
        timeout: Option<Duration>,
        headers: Vec<(String, String)>,
    }

    // Required setters: each one flips exactly ONE type parameter and keeps the other.
    impl<U, M> RequestBuilder<U, M> {
        pub fn url(self, url: impl Into<String>) -> RequestBuilder<String, M> {
            RequestBuilder {
                url: url.into(),
                method: self.method,
                timeout: self.timeout,
                headers: self.headers,
            }
        }

        pub fn method(self, method: Method) -> RequestBuilder<U, Method> {
            RequestBuilder {
                url: self.url,
                method,
                timeout: self.timeout,
                headers: self.headers,
            }
        }

        pub fn timeout(mut self, timeout: Duration) -> Self {
            self.timeout = Some(timeout);
            self
        }

        pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.headers.push((key.into(), value.into()));
            self
        }
    }

    //  `build()` exists ONLY when both required fields are set.
    impl RequestBuilder<String, Method> {
        pub fn build(self) -> Request {
            Request {
                url: self.url,
                method: self.method,
                timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
                headers: self.headers,
            }
        }
    }
}

use request::{Method, Request};
use std::time::Duration;

fn main() {
    // Order of the setters does not matter.
    let req = Request::builder()
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .url("https://example.com/api")
        .timeout(Duration::from_secs(5))
        .build();
    println!("{:?}", req);
    println!("headers: {:?}", req.headers());
}
