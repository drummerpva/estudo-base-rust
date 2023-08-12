use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
#[test]
fn test_get_rustaceans() {
  let client = Client::new();
  let rustacean1 = create_test_rustacean(&client);
  let rustacean2 = create_test_rustacean(&client);

  let response = client
    .get("http://localhost:8000/rustaceans")
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let object: Value = response.json().unwrap();
  assert!(object.as_array().unwrap().contains(&rustacean1));
  assert!(object.as_array().unwrap().contains(&rustacean2));
  delete_rustacean(&client, rustacean1);
  delete_rustacean(&client, rustacean2);
}
#[test]
fn test_create_rustacean() {
  let client = Client::new();
  let response = client
    .post("http://localhost:8000/rustaceans")
    .json(&json!({
      "name": "Foo bar",
      "email": "foo@bar.com"
    }))
    .send()
    .unwrap();
  assert_eq!(response.status(), StatusCode::CREATED);

  let object = response.json::<Value>().unwrap();
  assert_eq!(
    object,
    json!({
      "id": object["id"],
      "name": "Foo bar",
      "email": "foo@bar.com",
      "created_at": object["created_at"]
    })
  );
  delete_rustacean(&client, object);
}

#[test]
fn test_view_rustacean() {
  let client = Client::new();
  let create_object = create_test_rustacean(&client);

  let view_url = format!("http://localhost:8000/rustaceans/{}", create_object["id"]);
  let view_response = client.get(view_url).send().unwrap();
  assert_eq!(view_response.status(), StatusCode::OK);
  let view_object = view_response.json::<Value>().unwrap();

  assert_eq!(
    view_object,
    json!({
      "id": create_object["id"],
      "name": "Foo bar",
      "email": "foo@bar.com",
      "created_at": view_object["created_at"]
    })
  );
  delete_rustacean(&client, view_object);
}

#[test]
fn test_update_rustacean() {
  let client = Client::new();
  let create_object = create_test_rustacean(&client);

  let view_url = format!("http://localhost:8000/rustaceans/{}", create_object["id"]);
  let update_response = client
    .put(view_url)
    .json(&json!({
      "name": "Bar Foo",
      "email": "bar@foo.com"
    }))
    .send()
    .unwrap();
  assert_eq!(update_response.status(), StatusCode::OK);
  let update_object = update_response.json::<Value>().unwrap();

  assert_eq!(
    update_object,
    json!({
      "id": create_object["id"],
      "name": "Bar Foo",
      "email": "bar@foo.com",
      "created_at": update_object["created_at"]
    })
  );
  delete_rustacean(&client, update_object);
}

#[test]
fn test_delete_rustacean() {
  let client = Client::new();
  let create_object = create_test_rustacean(&client);

  let view_url = format!("http://localhost:8000/rustaceans/{}", create_object["id"]);
  let update_response = client.delete(&view_url).send().unwrap();
  assert_eq!(update_response.status(), StatusCode::NO_CONTENT);

  let view_response = client.get(&view_url).send().unwrap();

  assert_eq!(view_response.status(), StatusCode::NOT_FOUND);
}

fn delete_rustacean(client: &Client, rustacean: Value) {
  let view_url = format!("http://localhost:8000/rustaceans/{}", rustacean["id"]);
  client.delete(&view_url).send().unwrap();
}

fn create_test_rustacean(client: &Client) -> Value {
  let create_response = client
    .post("http://localhost:8000/rustaceans")
    .json(&json!({
      "name": "Foo bar",
      "email": "foo@bar.com"
    }))
    .send()
    .unwrap();
  create_response.json::<Value>().unwrap()
}
