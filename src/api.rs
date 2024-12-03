//! The application's API routes, controllers, DTOs, etc

use axum::extract::FromRef;

use crate::models::forum::Forum;

pub(crate) mod routes;

#[derive(Clone)]
pub struct ApiState {
    pub inner: std::sync::Arc<ApiStateInner>,
}

pub struct ApiStateInner {
    pub forums: Vec<Forum>,
}

macro_rules! impl_from_ref {
    ($($field:ident => $t:ty);+ $(;)?) => {
        $(
            impl FromRef<ApiState> for $t {
                fn from_ref(state: &ApiState) -> Self {
                    state.inner.$field.clone()
                }
            }
        )+
    };
}

impl_from_ref! {
    forums => Vec<Forum>;
}
