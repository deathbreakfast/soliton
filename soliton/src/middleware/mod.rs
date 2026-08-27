//! Per-request state injection for Axum apps (engine-agnostic).
//!
//! Hosts implement [`RequestExtensionState`] and layer [`attach_request_extensions`] so
//! handlers can extract injected values via `axum::Extension<T>` without this crate
//! depending on concrete engine types.
//!
//! ## Per-request extensions
//!
//! Handlers often need host-owned handles without coupling this crate to your engine types.
//! Layer [`attach_request_extensions`] on the router so it runs on **every per-request**
//! dispatch and copies values from your [`RequestExtensionState`] into Axum request extensions.
//!
//! **Prerequisites:** A cloneable host state type implementing [`RequestExtensionState`].
//!
//! ```rust,no_run
//! use axum::middleware::from_fn_with_state;
//! use axum::Router;
//! use soliton::middleware::{attach_request_extensions, RequestExtensionState};
//!
//! #[derive(Clone)]
//! struct AppState {
//!     label: String,
//! }
//!
//! impl RequestExtensionState for AppState {
//!     fn inject_request_extensions(&self, extensions: &mut axum::http::Extensions) {
//!         extensions.insert(self.label.clone());
//!     }
//! }
//!
//! let state = AppState { label: "host-1".into() };
//! let app: Router<()> = Router::new().layer(from_fn_with_state(
//!     state.clone(),
//!     attach_request_extensions::<AppState>,
//! ));
//! let mut extensions = axum::http::Extensions::new();
//! state.inject_request_extensions(&mut extensions);
//! assert_eq!(extensions.get::<String>(), Some(&"host-1".to_string()));
//! assert!(std::mem::size_of_val(&app) > 0);
//! ```
//!
//! Wire the middleware with `from_fn_with_state` (see `# Examples` below). Runnable:
//! `cargo run -p soliton --example process_host`
//!
//! # Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Declare what to inject per request | [`RequestExtensionState`] |
//! | Axum middleware that performs the inject | [`attach_request_extensions`] |
//!
//! # Examples
//!
//! ```rust,no_run
//! use axum::middleware::from_fn_with_state;
//! use axum::Router;
//! use soliton::middleware::{attach_request_extensions, RequestExtensionState};
//!
//! #[derive(Clone)]
//! struct AppState {
//!     label: String,
//! }
//!
//! impl RequestExtensionState for AppState {
//!     fn inject_request_extensions(&self, extensions: &mut axum::http::Extensions) {
//!         extensions.insert(self.label.clone());
//!     }
//! }
//!
//! let app: Router<()> = Router::new().layer(from_fn_with_state(
//!     AppState { label: "host-1".into() },
//!     attach_request_extensions::<AppState>,
//! ));
//! assert!(std::mem::size_of_val(&app) > 0);
//! ```
//!
//! [`RequestExtensionState`]: crate::middleware::RequestExtensionState
//! [`attach_request_extensions`]: crate::middleware::attach_request_extensions

/// See [`RequestExtensionState`] and [`attach_request_extensions`].
pub mod request_context;

pub use request_context::{attach_request_extensions, RequestExtensionState};
