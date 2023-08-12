use actix_web::{
  delete, get, post,
  web::{Data, Path},
  HttpResponse,
};
use chrono::NaiveDateTime;
// use diesel::prelude::*;
use diesel::{
  r2d2::{ConnectionManager, Pool},
  Insertable, MysqlConnection, Queryable, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[table_name = "crate::schema::likes"]
pub struct LikeAdd {
  pub id: Option<u32>,
  pub created_at: Option<NaiveDateTime>,
  pub tweet_id: u32,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct Like {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub tweet_id: i32,
}
impl LikeAdd {
  pub fn new(tweet_id: u32) -> Self {
    Self {
      tweet_id,
      id: None,
      created_at: None,
    }
  }
}

#[get("/tweets/{id}/likes")]
pub async fn get_likes(path: Path<(String,)>) -> HttpResponse {
  let _tweet_founded = format!("have 1 like the tweet: {}", path.0);
  let likes = 103;
  HttpResponse::Ok() //200
    .json(likes)
}
#[post("/tweets/{id}/likes")]
pub async fn add_like(
  path: Path<(String,)>,
  pool: Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> HttpResponse {
  let tweet_id = path.0.parse::<u32>().unwrap();
  let like_add = LikeAdd::new(tweet_id);
  let mut conn = pool
    .get()
    .expect("Não foi possível conectar a base de dados");
  diesel::insert_into(crate::schema::likes::table)
    .values(&like_add)
    .execute(&mut conn)
    .expect("Erro ao inserir like");
  HttpResponse::Created() //201
    .json(tweet_id)
}
#[delete("/tweets/{id}/likes")]
pub async fn delete_like(path: Path<(String,)>) -> HttpResponse {
  let _tweet_founded = format!("have 1 like the tweet: {}", path.0);
  HttpResponse::NoContent() //204
    .await
    .unwrap()
}
