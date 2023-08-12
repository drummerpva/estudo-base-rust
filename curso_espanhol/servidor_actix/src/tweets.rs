use super::schema::tweets::{self, dsl::tweets as tweets_dsl};
use actix_web::{
  get, post,
  web::{Data, Path},
  HttpResponse,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{
  self,
  r2d2::{ConnectionManager, Pool},
  Insertable, MysqlConnection, Queryable, QueryableByName, RunQueryDsl, Selectable, Table,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[table_name = "tweets"]
struct Tweet {
  id: Option<u32>,
  created_at: Option<NaiveDateTime>,
  message: String,
}
#[derive(Queryable, Serialize, Deserialize)]
struct TweetSelect {
  id: u32,
  created_at: NaiveDateTime,
  message: String,
}
#[derive(Debug, QueryableByName)]
#[table_name = "diesel::sql_types::Integer"]
struct Id {
  #[sql_type = "diesel::sql_types::Integer"]
  id: i32,
}

impl Tweet {
  fn new(message: String) -> Self {
    Self {
      id: None,
      // created_at: Some(Utc::now().naive_utc()),
      created_at: None,
      message,
    }
  }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<MysqlConnection>>>) -> HttpResponse {
  use crate::schema::tweets::dsl::*;
  let mut conn = pool
    .get()
    .expect("Não foi possível conectar a base de dados");
  let result = tweets
    .order(created_at.desc())
    .limit(10)
    .load::<TweetSelect>(&mut conn);
  let response = match result {
    Ok(tws) => tws,
    Err(_) => vec![],
  };
  HttpResponse::Ok().json(response)
}

#[post("/tweets")]
pub async fn create_tweet(
  req_body: String,
  pool: Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> HttpResponse {
  let new_tweet = Tweet::new(req_body);
  let mut conn = pool.get_ref().get().unwrap();
  /*   diesel::sql_query("INSERT INTO tweets (message) VALUES (?)")
  .bind::<diesel::sql_types::Text, _>(&new_tweet.message)
  .execute(&mut conn)
  .unwrap(); */
  diesel::insert_into(tweets::table)
    .values(&new_tweet)
    .execute(&mut conn)
    .expect("Error saving new tweet");
  let last_inserted_id: Id = diesel::sql_query("SELECT LAST_INSERT_ID() as id")
    .get_result(&mut conn)
    .unwrap();
  let new_tweet_db: TweetSelect = tweets_dsl
    .find(last_inserted_id.id as u32)
    .select(tweets_dsl::all_columns())
    .first(&mut conn)
    .unwrap();
  HttpResponse::Created() //201
    .json(new_tweet_db)
}

#[get("/tweets/{id}")]
pub async fn get_tweet(path: Path<(String,)>) -> HttpResponse {
  let tweet_founded = format!("this is tweet founded: {}", path.0);
  HttpResponse::Ok() //200
    .json(tweet_founded)
}
