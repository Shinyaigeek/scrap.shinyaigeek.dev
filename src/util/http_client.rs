use hyper::body::to_bytes;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;

pub struct HttpClient {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

pub struct HttpResponse {
    pub body: String,
}

impl HttpClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        return Self {
            client: Client::builder().build::<_, hyper::Body>(https),
        };
    }

    pub async fn get(self, url: &str) -> HttpResponse {
        let uri = url.parse::<Uri>().unwrap();
        let res = self.client.get(uri).await;
        let mut res = res.unwrap().into_body();
        let body = to_bytes(res).await;
        let body = body.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        HttpResponse { body }
    }

    pub async fn post(self, url: &str, payload: HashMap<String, String>) -> HttpResponse {
        let uri = url.parse::<Uri>().unwrap();
        let request_body = {
            let mut body = String::from("{");
            for (key, value) in payload {
                body.push_str("\"");
                body.push_str(&key);
                body.push_str("\"");
                body.push_str(":");
                body.push_str("\"");
                body.push_str(&value);
                body.push_str("\"");
                // body.push_str(",");
            }
            body.push_str("}");

            body
        };
        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header("content-type", "application/json")
            .body(Body::from(request_body))
            .expect("request builder");

        let res = self.client.request(req).await;
        let mut res = res.unwrap().into_body();
        let body = to_bytes(res).await;
        let body = body.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        HttpResponse { body }
    }
}
