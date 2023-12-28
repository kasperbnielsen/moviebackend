use axum::response::IntoResponse;

use crate::movie::models::Details;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Watchlist {
    pub user_id: String,
    pub movie_id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ListWatchlist {
    pub id_list: Vec<String>,
    pub user_id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IsWatchlisted {
    pub is_watchlisted: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]

pub struct DetailsList {
    pub list: Vec<Details>,
}

impl IntoResponse for IsWatchlisted {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for ListWatchlist {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for DetailsList {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
