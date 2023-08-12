// /tweets -> GET lista tweets, POST create a tweet
// tweets/:id -> GET return a tweet, DELETE delete a tweet
// /tweets/:id/likes -> GET get likes from tweet, POST create a like, DELETE delete a like
#[macro_use]
extern crate diesel;

use actix_web::{web::Data, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
mod likes;
mod schema;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não configurada");
  let manager = ConnectionManager::<diesel::MysqlConnection>::new(database_url);
  let pool = Pool::builder()
    .build(manager)
    .expect("Falha ao criar pool de conexões");
  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(pool.clone()))
      // .app_data(web::JsonConfig::default().limit(1024 * 16))
      // .route("/ola", web::get().to(hello))
      .service(tweets::get_tweets)
      .service(tweets::create_tweet)
      .service(tweets::get_tweet)
      .service(likes::get_likes)
      .service(likes::add_like)
      .service(likes::delete_like)
  })
  .bind("127.0.0.1:4002")?
  .run()
  .await
}

/* async fn _hello() -> impl Responder {
  HttpResponse::Ok().body("Hello from actix!")
}
 */
