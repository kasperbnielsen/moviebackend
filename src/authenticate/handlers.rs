use axum::{extract::State, http::StatusCode, Json};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
    Collection,
};

use crate::get_collection;

use super::models::{User, UserDTO, UserInput};

pub async fn create_user(
    State(database): State<mongodb::Client>,
    Json(payload): Json<UserDTO>,
) -> Result<(), StatusCode> {
    let collection: Collection<UserDTO> = get_collection(database, "users");

    let user = collection
        .insert_one(
            UserDTO {
                username: payload.username,
                password: payload.password,
            },
            None,
        )
        .await;

    match user {
        Ok(_) => Ok(()),
        Err(_err) => Err(StatusCode::NOT_ACCEPTABLE),
    }
}

pub async fn authenticate(
    State(database): State<mongodb::Client>,
    Json(payload): Json<UserDTO>,
) -> Result<User, StatusCode> {
    let collection: Collection<UserInput> = get_collection(database, "users");

    let user = collection
        .find_one(doc! {"username": payload.username}, None)
        .await;

    match user {
        Ok(Some(new_user)) => {
            if (new_user.password != payload.password) {
                return Err(StatusCode::NOT_ACCEPTABLE);
            }
            Ok(User {
                username: new_user.username,
                password: new_user.password,
                _id: new_user._id.to_hex(),
            })
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("{:?}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
