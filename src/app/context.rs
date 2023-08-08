use crate::app::config::Config;
use axum::extract::FromRequestParts;
use http::request::Parts;
use http::StatusCode;
use std::ops::Deref;

use std::sync::{Arc, OnceLock};

/// Application level context
///
/// This struct contains things that should be available for the whole process lifetime.
pub struct AppContext {
    pub config: Arc<Config>,
}

/// A new-type of `AppContext`
///
/// See `AppContext::into_ref` for it usage.
#[derive(Copy, Clone)]
pub struct RefContext(&'static AppContext);

impl RefContext {
    #[allow(dead_code)]
    pub fn inner(self) -> &'static AppContext {
        self.0
    }
}

impl AppContext {
    pub async fn create(config: Config) -> Self {
        let config = Arc::new(config);
        AppContext { config }
    }

    /// Turn `AppContext` into `RefContext` which is easy to work with because it implements `Copy`
    ///
    /// This method should be called once. The first successfully cache context will be leaked
    pub fn into_ref(self) -> RefContext {
        static CONTEXT: OnceLock<AppContext> = OnceLock::new();
        RefContext(CONTEXT.get_or_init(|| self))
    }
}

impl AsRef<AppContext> for RefContext {
    fn as_ref(&self) -> &AppContext {
        self.0
    }
}

impl Deref for RefContext {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for RefContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Self>()
            .copied()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
