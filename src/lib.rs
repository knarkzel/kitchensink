// Modules
pub mod discord;
pub mod feeds;
pub mod home;
pub mod http;
pub mod navbar;
pub mod settings;
pub mod supabase;
pub mod account;

// Imports
pub use anyhow::Result;
pub use dioxus::prelude::*;
pub use dioxus_router::*;
pub use fermi::prelude::*;
pub use gloo_storage::{LocalStorage, Storage};
pub use serde_json::json;

// Constants
pub const SUPABASE_ENDPOINT: &str = "https://ojjkwixnlbiakfnhaksp.supabase.co";
pub const SUPABASE_ANON_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im9qamt3aXhubGJpYWtmbmhha3NwIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzczMzMyODksImV4cCI6MTk5MjkwOTI4OX0.nVHH5sgXztN-3eweYHOD5eGi6g7Msj2U4RxJ0pCtHiU";

// Global state
pub static USER: Atom<Option<SupabaseUser>> = |_| None;
pub static SETTINGS: Atom<Settings> = |_| Settings::default();

// Types
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub feeds: String,
}
