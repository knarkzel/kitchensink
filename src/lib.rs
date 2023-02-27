// Modules
pub mod discord;
pub mod feeds;
pub mod home;
pub mod login;
pub mod navbar;
pub mod settings;
pub mod signup;

// Imports
pub use dioxus::prelude::*;
pub use dioxus_router::*;
pub use fermi::prelude::*;
pub use gloo_storage::{LocalStorage, Storage};
pub use serde_json::json;

// User
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupabaseUser {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

// Global state
pub static USER: Atom<Option<SupabaseUser>> = |_| None;

// Constants
const SUPABASE_ENDPOINT: &str = "https://ojjkwixnlbiakfnhaksp.supabase.co";
const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im9qamt3aXhubGJpYWtmbmhha3NwIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzczMzMyODksImV4cCI6MTk5MjkwOTI4OX0.nVHH5sgXztN-3eweYHOD5eGi6g7Msj2U4RxJ0pCtHiU";

// Supabase
pub struct Supabase;

impl Supabase {
    pub fn new() -> postgrest::Postgrest {
        postgrest::Postgrest::new(format!("{SUPABASE_ENDPOINT}/rest/v1"))
            .insert_header("apikey", SUPABASE_ANON_KEY)
    }
}

// Regular HTTP client
use reqwest::Response;
use std::collections::HashMap;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn signup(&self, email: &str, password: &str) -> Result<Response, reqwest::Error> {
        let body = HashMap::from([("email", email), ("password", password)]);
        self.client
            .post(format!("{SUPABASE_ENDPOINT}/auth/v1/signup"))
            .header("apikey", SUPABASE_ANON_KEY)
            .json(&body)
            .send()
            .await
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<Response, reqwest::Error> {
        let body = HashMap::from([("email", email), ("password", password)]);
        self.client
            .post(format!("{SUPABASE_ENDPOINT}/auth/v1/token"))
            .header("apikey", SUPABASE_ANON_KEY)
            .query(&[("grant_type", "password")])
            .json(&body)
            .send()
            .await
    }
}
