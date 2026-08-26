//! Soliton turns a composed Axum [`Router`](axum::Router) into a listening host process.
//! It resolves and binds a TCP address, runs the accept loop, merges a `/health` probe route,
//! and optionally layers subsystem HMAC auth and per-request extension injection before your
//! handlers run. Bring routes and state; Soliton handles process boot, listen policy, and the
//! shared middleware stack around them.
//!
//! Operational events use the [`tracing`] crate (bind, serve lifecycle, HMAC rejects,
//! worker-stack env warnings). **Hosts own the subscriber** — without one, events are
//! no-ops.
//!
//! # Features
//!
//! - **HTTP bind and serve** — Resolve a listen address, enforce bind policy, and run an Axum accept loop. [Get started](#bind-and-serve)
//! - **Request extensions** — Inject host-owned values into each request for handlers to extract. [Get started](crate::middleware#per-request-extensions)
//! - **Health probe** — Expose `GET /health` for load balancers and orchestrators. [Get started](crate::health#liveness-probe)
//! - **Subsystem HMAC** — Require signed bearer headers on subsystem API paths when a key is configured. [Get started](crate::subsystem_auth#subsystem-hmac)
//! - **Tokio runtime** — Boot the host with enlarged worker stacks for deep async call chains. [Get started](crate::tokio_runtime#host-runtime)
//!
//! # Getting started
//!
//! Runnable host wire-up: `cargo run -p soliton --example process_host`
//!
//! Auth contract smoke: `cargo run -p soliton --example hmac_health_host`
//!
//! ## Bind and serve
//!
//! Soliton turns a composed Axum [`Router`](axum::Router) into a listening TCP host. At
//! **host boot**, call this path from `main` after routes, health, and middleware layers are
//! wired — it resolves the listen address, applies bind policy, and blocks in the accept loop
//! until the process exits.
//!
//! **Prerequisites:** An Axum [`Router<()>`](axum::Router) with [`Router::with_state`](axum::Router::with_state)
//! applied when handlers need state. Non-loopback binds require `SUBSYSTEM_AUTH_HMAC_KEY`
//! (≥ 32 bytes) through [`bind_tcp_with_policy`].
//!
//! ```rust,no_run
//! use axum::{middleware::from_fn, routing::get, Router};
//! use soliton::listener::{bind_tcp_with_policy, resolve_listen_addr, ListenAddrDefault};
//! use soliton::subsystem_auth::axum_optional_subsystem_hmac;
//! use soliton::{health_router, serve};
//!
//! fn app() -> Router {
//!     Router::new()
//!         .route("/api/ping", get(|| async { axum::http::StatusCode::OK }))
//!         .merge(health_router())
//!         .layer(from_fn(axum_optional_subsystem_hmac))
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     soliton::tokio_runtime::run(async {
//!         let addr = resolve_listen_addr(ListenAddrDefault::Loopback { port: 3002 })?;
//!         assert!(addr.ip().is_loopback());
//!         let listener = bind_tcp_with_policy(addr).await?;
//!         assert!(listener.local_addr()?.port() > 0);
//!         serve(listener, app()).await?;
//!         Ok::<(), Box<dyn std::error::Error>>(())
//!     })
//! }
//! ```
//!
//! **Next:** [`process_host`](../../examples/process_host.rs) for extensions and graceful shutdown.
//! For unsigned `/api` rejection when HMAC is configured, see
//! [Subsystem HMAC — unsigned requests](crate::subsystem_auth#unsigned-api-requests).
//!
//! # Further reading
//!
//! - Crate README — dependency snippet and About inventory
//! - `docs/VERIFICATION.md` — test map and doc gates
//! - `SECURITY.md` — host composition and subsystem HMAC requirements

/// Typed listen and bind-policy errors.
pub mod error;
/// `GET /health` router (see [`health_router`]).
pub mod health;
mod http_serve;
/// TCP listener + bind-address resolution helpers.
pub mod listener;
/// Per-request state injection middleware (see [`middleware::RequestExtensionState`]).
pub mod middleware;
/// Optional HMAC bearer middleware for headless subsystem HTTP APIs.
pub mod subsystem_auth;
/// Multi-thread Tokio runtime helper with a configurable per-worker stack.
pub mod tokio_runtime;

pub use error::{BindPolicyError, ListenAddrError};
pub use health::health_router;
pub use http_serve::{serve, serve_with_graceful_shutdown};
pub use listener::{
    bind_tcp, bind_tcp_with_policy, ensure_bind_allowed, resolve_listen_addr, ListenAddrDefault,
};
