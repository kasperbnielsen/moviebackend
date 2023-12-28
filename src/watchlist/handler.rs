use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures::StreamExt;
use mongodb::{bson::doc, Collection};
use reqwest::header::AUTHORIZATION;
use serde::de::value;

use crate::{get_collection, movie::models::Details};

use super::models::{DetailsList, IsWatchlisted, ListWatchlist, Watchlist};

pub async fn add_to_watchlist(
    State(database): State<mongodb::Client>,
    Json(payload): Json<Watchlist>,
) -> Result<(), StatusCode> {
    let collection = get_collection(database, "watchlists");

    match collection
        .insert_one(
            doc! { "movie_id": payload.movie_id, "user_id": payload.user_id},
            None,
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("{:?}", err);
            Err(StatusCode::NOT_ACCEPTABLE)
        }
    }
}

pub async fn is_watchlisted(
    State(database): State<mongodb::Client>,
    Json(payload): Json<Watchlist>,
) -> Result<IsWatchlisted, StatusCode> {
    let collection: Collection<Watchlist> = get_collection(database, "watchlists");

    match collection
        .find_one(
            doc! { "user_id": payload.user_id, "movie_id": payload.movie_id},
            None,
        )
        .await
    {
        Ok(_) => Ok(IsWatchlisted {
            is_watchlisted: true,
        }),
        Err(_) => Ok(IsWatchlisted {
            is_watchlisted: false,
        }),
    }
}

pub async fn remove_from_watchlist(
    State(database): State<mongodb::Client>,
    Json(payload): Json<Watchlist>,
) -> Result<(), StatusCode> {
    let collection: Collection<Watchlist> = get_collection(database, "watchlists");

    match collection
        .delete_one(
            doc! { "movie_id": payload.movie_id, "user_id": payload.user_id},
            None,
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("{:?}", err);
            Err(StatusCode::NOT_ACCEPTABLE)
        }
    }
}

pub async fn get_watchlist(
    State(database): State<mongodb::Client>,
    Path(user_id): Path<String>,
) -> Result<DetailsList, StatusCode> {
    let collection: Collection<Watchlist> = get_collection(database, "watchlists");

    let mut rows: Vec<String> = Vec::new();

    let movies = collection.find(doc! {"user_id": &user_id}, None).await;

    println!("hello");

    match movies {
        Ok(mut value) => {
            while let Some(Ok(temp)) = value.next().await {
                rows.push(temp.movie_id)
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(StatusCode::NOT_FOUND);
        }
    }

    println!("{:?}", rows);
    let auth_header: &str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI3MzU2ZjZjNzgxZjg0MjAyNjM2N2I4YmFhMjI1YWJkYiIsInN1YiI6IjY1MDFjOTdkNTU0NWNhMDBhYjVkYmRkOSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.zvglGM1QgLDK33Dt6PpMK9jeAOrLNnxClZ6mkLeMgBE";

    let mut id_list: Vec<Details> = Vec::new();

    for id in rows {
        let url = "https://api.themoviedb.org/3/movie/".to_owned() + &id + "?language=en-US";

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await;

        match response {
            Ok(value) => match value.json::<Details>().await {
                Ok(value) => id_list.push(value),
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err(StatusCode::IM_A_TEAPOT);
                }
            },
            Err(err) => {
                eprintln!("{:?}", err);
                return Err(StatusCode::NOT_FOUND);
            }
        }
    }

    Ok(DetailsList { list: id_list })
}
