use crate::*;
use reqwest::Response;
use std::collections::HashMap;

pub struct Client(reqwest::Client);

impl Client {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub async fn signup(&self, email: &str, password: &str) -> Result<Response, reqwest::Error> {
        let body = HashMap::from([("email", email), ("password", password)]);
        self.0
            .post(format!("{SUPABASE_ENDPOINT}/auth/v1/signup"))
            .header("apikey", SUPABASE_ANON_KEY)
            .json(&body)
            .send()
            .await
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<Response, reqwest::Error> {
        let body = HashMap::from([("email", email), ("password", password)]);
        self.0
            .post(format!("{SUPABASE_ENDPOINT}/auth/v1/token"))
            .header("apikey", SUPABASE_ANON_KEY)
            .query(&[("grant_type", "password")])
            .json(&body)
            .send()
            .await
    }
}
