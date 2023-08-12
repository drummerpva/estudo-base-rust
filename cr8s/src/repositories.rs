use diesel::prelude::*;

use crate::{models::*, schema::*};

pub struct RustaceanRepository<'a> {
  pub connection: &'a mut PgConnection,
}

impl<'a> RustaceanRepository<'a> {
  pub fn find(&mut self, id: i32) -> QueryResult<Rustacean> {
    rustaceans::table.find(id).get_result(&mut *self.connection)
  }
  pub fn find_multiple(&mut self, limit: i64) -> QueryResult<Vec<Rustacean>> {
    rustaceans::table.limit(limit).load(&mut *self.connection)
  }
  pub fn create(&mut self, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
    diesel::insert_into(rustaceans::table)
      .values(&new_rustacean)
      .get_result(&mut *self.connection)
  }
  pub fn update(&mut self, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
    diesel::update(rustaceans::table.find(id))
      .set((
        rustaceans::name.eq(rustacean.name),
        rustaceans::email.eq(rustacean.email),
      ))
      .get_result(&mut *self.connection)
  }
  pub fn delete(&mut self, id: i32) -> QueryResult<usize> {
    diesel::delete(rustaceans::table.find(id)).execute(&mut *self.connection)
  }
}

pub struct CrateRepository<'a> {
  pub connection: &'a mut PgConnection,
}

impl<'a> CrateRepository<'a> {
  pub fn find(&mut self, id: i32) -> QueryResult<Crate> {
    crates::table.find(id).get_result(&mut *self.connection)
  }
  pub fn find_multiple(&mut self, limit: i64) -> QueryResult<Vec<Crate>> {
    crates::table.limit(limit).load(&mut *self.connection)
  }
  pub fn create(&mut self, new_crate: NewCrate) -> QueryResult<Crate> {
    diesel::insert_into(crates::table)
      .values(&new_crate)
      .get_result(&mut *self.connection)
  }
  pub fn update(&mut self, id: i32, new_crate: Crate) -> QueryResult<Crate> {
    diesel::update(crates::table.find(id))
      .set((
        crates::rustacean_id.eq(new_crate.rustacean_id),
        crates::code.eq(new_crate.code),
        crates::name.eq(new_crate.name),
        crates::version.eq(new_crate.version),
        crates::description.eq(new_crate.description),
      ))
      .get_result(&mut *self.connection)
  }
  pub fn delete(&mut self, id: i32) -> QueryResult<usize> {
    diesel::delete(crates::table.find(id)).execute(&mut *self.connection)
  }
}
