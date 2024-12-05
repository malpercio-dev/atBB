use axum::{extract::State, Json};

use crate::models::forum::Forum;

pub async fn get_forums(State(forums): State<Vec<Forum>>) -> Json<Vec<Forum>> {
    Json(forums)
}
