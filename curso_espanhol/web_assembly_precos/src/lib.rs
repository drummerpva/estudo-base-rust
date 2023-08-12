use serde_json::{Map, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

#[wasm_bindgen]
pub async fn get_coin_price(coin: String) -> Result<String, JsValue> {
  let mut opts: RequestInit = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let url = format!("https://api.coingecko.com/api/v3/coins/{coin}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false");
  let request = Request::new_with_str_and_init(&url, &opts)?;
  let window: Window = web_sys::window().unwrap();
  let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Response = response_value.dyn_into().unwrap();
  let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();
  let price = parse_body(&text);

  Ok(price)
}

fn parse_body(body: &str) -> String {
  let json: Map<String, Value> = serde_json::from_str(body).unwrap();
  json["market_data"]["current_price"]["usd"].to_string()
}
