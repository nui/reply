#![allow(clippy::redundant_clone)]

use axum::extract::{DefaultBodyLimit, Extension};
use axum::routing::get;
use axum::Router;
use http::Request;
use tracing::{Level, Span};

use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;

use crate::app::context::{AppContext, RefContext};
use crate::handlers as H;

/// Main application router
fn main_route(context: RefContext) -> Router<RefContext> {
    let AppContext { config: _ } = context.as_ref();

    // Start with routing which should be in all rp provider
    #[rustfmt::skip]
    let routes = vec![
        ("/", get(H::root)),
        ("/epoch", get(H::epoch)),
    ];

    routes
        .into_iter()
        .fold(Router::new(), |router, (path, service)| {
            router.route(path, service)
        })
}

pub fn create_router(context: RefContext) -> Router<RefContext> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(new_make_span)
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        );
    let middlewares = ServiceBuilder::new()
        .layer(Extension(context))
        .layer(trace_layer)
        .layer(DefaultBodyLimit::disable());
    main_route(context).layer(middlewares)
}

fn new_make_span<Body>(_: &Request<Body>) -> Span {
    Span::none()
}
