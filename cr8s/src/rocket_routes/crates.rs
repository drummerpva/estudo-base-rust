use rocket::{
  http::Status,
  response::status::{Custom, NoContent},
  serde::json::{serde_json::json, Json, Value},
};

use crate::{
  models::{Crate, NewCrate},
  repositories::CrateRepository,
};

use super::DbConnection;

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConnection) -> Result<Value, Custom<Value>> {
  db.run(|connection| {
    let mut repository = CrateRepository { connection };
    repository
      .find_multiple(100)
      .map(|row| json!(row))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConnection) -> Result<Value, Custom<Value>> {
  db.run(move |connection| {
    let mut repository = CrateRepository { connection };
    repository.find(id).map(|row| json!(row)).map_err(|err| {
      println!("Error: {:?}", err);
      Custom(Status::InternalServerError, json!("Error"))
    })
  })
  .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
  new_crate: Json<NewCrate>,
  db: DbConnection,
) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |connection| {
    let mut repository = CrateRepository { connection };
    repository
      .create(new_crate.into_inner())
      .map(|row| Custom(Status::Created, json!(row)))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<crate_model>")]
pub async fn update_crate(
  id: i32,
  crate_model: Json<Crate>,
  db: DbConnection,
) -> Result<Value, Custom<Value>> {
  db.run(move |connection| {
    let mut repository = CrateRepository { connection };
    repository
      .update(id, crate_model.into_inner())
      .map(|row| json!(row))
      .map_err(|err| {
        println!("Error: {:?}", err);
        Custom(Status::InternalServerError, json!("Error"))
      })
  })
  .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConnection) -> Result<NoContent, Custom<Value>> {
  db.run(move |connection| {
    let mut repository = CrateRepository { connection };
    repository.delete(id).map(|_| NoContent).map_err(|err| {
      println!("Error: {:?}", err);
      Custom(Status::InternalServerError, json!("Error"))
    })
  })
  .await
}
