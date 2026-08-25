//! `GET /health` router for load balancers and orchestrators.
//!
//! ## Liveness probe
//!
//! [`health_router`] adds a probe route that returns `200 OK` without touching authenticated
//! API paths. Merge it when you build the host app, before [`crate::serve`] runs at boot.
//!
//! **Prerequisites:** An Axum router ready to merge; works with any router state via generics.
//!
//! ```rust
//! use axum::body::Body;
//! use axum::http::{Request, StatusCode};
//! use axum::Router;
//! use soliton::health_router;
//! use tower::ServiceExt;
//!
//! # async fn demo() {
//! let app = Router::new().merge(health_router());
//! let response = app
//!     .oneshot(
//!         Request::builder()
//!             .uri("/health")
//!             .body(Body::empty())
//!             .unwrap(),
//!     )
//!     .await
//!     .unwrap();
//! assert_eq!(response.status(), StatusCode::OK);
//! # }
//! ```

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

/// `GET /health` returning `200 OK`.
///
/// Generic over router state so hosts can `.merge(health_router())` into a
/// stateful `Router<S>` (Axum 0.8 does not coerce `Router<()>` via `From`).
///
/// # Examples
///
/// ```rust,no_run
/// use axum::Router;
/// use soliton::health_router;
///
/// let app: Router<()> = Router::new().merge(health_router());
/// assert!(std::mem::size_of_val(&app) > 0);
/// ```
pub fn health_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(|| async { StatusCode::OK }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_route_returns_ok_happy_path() {
        let app = health_router::<()>();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
