use crate::get_collection;
use axum::{
    extract::{Path, State},
    Json,
};
use futures::StreamExt;
use mongodb::{bson::doc, Collection};
use reqwest::StatusCode;

use super::models::{Comment, CommentsList};

pub async fn get_comments(
    State(database): State<mongodb::Client>,
    Path(movie_id): Path<String>,
) -> Result<CommentsList, StatusCode> {
    let collection: Collection<Comment> = get_collection(database, "comments");

    let mut rows: Vec<Comment> = Vec::new();

    let cursor = collection.find(doc! {"movie_id": movie_id}, None).await;

    match cursor {
        Ok(mut value) => {
            while let Some(Ok(temp)) = value.next().await {
                rows.push(temp)
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(StatusCode::NOT_FOUND);
        }
    }

    Ok(CommentsList { list: rows })
}

pub async fn post_comment(
    State(database): State<mongodb::Client>,
    Path(movie_id): Path<String>,
    Json(payload): Json<Comment>,
) -> Result<(), StatusCode> {
    let collection = get_collection(database, "comments");

    match collection
        .insert_one(
            doc! {
                "movie_id": movie_id,
                "user_id": payload.user_id,
                "comment": payload.comment,
            },
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
