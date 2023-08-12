use rusqlite::{Connection, Error};

fn main() {
  let connection = create_database();
  create_table(&connection);
  // insert_user(&connection, "John Doe");

  let user = get_user(&connection, 1).unwrap();
  println!("User: {:?}", user);
}

fn get_user(conn: &Connection, id: i32) -> Result<String, Error> {
  let mut statement = conn.prepare("SELECT name FROM users WHERE id = ?1")?;
  let users = statement.query_map([id], |row| {
    let name: String = row.get(0)?;
    Ok(name)
  })?;
  for user in users {
    return Ok(user.unwrap());
  }
  Ok(String::from("User not found!"))
}

fn insert_user(conn: &Connection, name: &str) {
  let result = conn.execute("INSERT INTO users (name) VALUES (?1)", [name]);
  match result {
    Ok(_) => println!("User inserted successfully"),
    Err(_) => panic!("User insertion failed"),
  }
}

fn create_table(conn: &Connection) {
  let result = conn.execute(
    "CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL
    )",
    [],
  );
  match result {
    Ok(_) => println!("Table created successfully"),
    Err(err) => panic!("Table creation failed: {:?}", err),
  }
}

fn create_database() -> Connection {
  let result = Connection::open("users.sqlite3");
  match result {
    Ok(_) => {
      println!("Database created successfully");
      result.unwrap()
    }
    Err(_) => panic!("Database creation failed"),
  }
}
