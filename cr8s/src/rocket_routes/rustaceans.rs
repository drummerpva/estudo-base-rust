use rocket::{
  http::Status,
  response::status::{Custom, NoContent},
  serde::json::{serde_json::json, Json, Value},
};

use crate::{models::*, repositories::RustaceanRepository};

use super::DbConnection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConnection) -> Result<Value, Custom<Value>> {
  db.run(|connection| {
    let mut rustacean_repository = RustaceanRepository { connection };
    rustacean_repository
      .find_multiple(100)
      .map(|rustacean| json!(rustacean))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConnection) -> Result<Value, Custom<Value>> {
  db.run(move |connection| {
    let mut rustacean_repository = RustaceanRepository { connection };
    rustacean_repository
      .find(id)
      .map(|rustacean| json!(rustacean))
      .map_err(|err| {
        println!("Error: {}", err.to_string());
        if err.to_string() == String::from("Record not found") {
          return Custom(Status::NotFound, json!("Not Found"));
        }
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
  new_rustacean: Json<NewRustacean>,
  db: DbConnection,
) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |connection| {
    let mut rustacean_repository = RustaceanRepository { connection };
    rustacean_repository
      .create(new_rustacean.into_inner())
      .map(|rustacean| Custom(Status::Created, json!(rustacean)))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
  id: i32,
  rustacean: Json<Rustacean>,
  db: DbConnection,
) -> Result<Value, Custom<Value>> {
  db.run(move |connection| {
    let mut rustacean_repository = RustaceanRepository { connection };
    rustacean_repository
      .update(id, rustacean.into_inner())
      .map(|rustacean| json!(rustacean))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConnection) -> Result<NoContent, Custom<Value>> {
  db.run(move |connection| {
    let mut rustacean_repository = RustaceanRepository { connection };
    rustacean_repository
      .delete(id)
      .map(|_| NoContent)
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}
