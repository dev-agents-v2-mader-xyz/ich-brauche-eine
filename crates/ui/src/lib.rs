// Suppress warnings for imports/variables only used inside #[cfg(target_arch = "wasm32")] blocks.
// This crate is a WASM UI library; native compilation is for tooling only.
#![cfg_attr(not(target_arch = "wasm32"), allow(unused_imports, unused_variables))]

pub mod api;
pub mod app;
pub mod auth;
pub mod components;
pub mod pages;
pub mod routes;
pub mod types;
pub mod utils;

pub use app::App;
