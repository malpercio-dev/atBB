//! The application's web server, static content, etc

use axum::extract::FromRef;

use crate::models::forum::Forum;

pub(crate) mod pages;

#[derive(Clone)]
pub struct WebState {
    pub inner: std::sync::Arc<WebStateInner>,
}

pub struct WebStateInner {
    pub forums: Vec<Forum>,
}

macro_rules! impl_from_ref {
    ($($field:ident => $t:ty);+ $(;)?) => {
        $(
            impl FromRef<WebState> for $t {
                fn from_ref(state: &WebState) -> Self {
                    state.inner.$field.clone()
                }
            }
        )+
    };
}

impl_from_ref! {
    forums => Vec<Forum>;
}
