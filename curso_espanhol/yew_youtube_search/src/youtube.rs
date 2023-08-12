use gloo_net::{http::Request, Error};
use serde::Deserialize;

use crate::env::API_KEY;

pub async fn search_youtube(text: String) -> Result<Video, Error> {
  let query_url = format!(
    "https://youtube.googleapis.com/youtube/v3/search?part=snippet&q={}&key={API_KEY}",
    text
  );
  // let auth = String::from(format!("Bearer {API_KEY}"));
  let response = Request::get(&query_url)
    // .header("Authorization", &auth)
    .header("Accept", "application/json")
    .send()
    .await?;
  let search_result: SearchResult = response.json().await?;
  let empty_video = empty_video();
  let video = match search_result.items.first() {
    Some(video) => video,
    None => &empty_video,
  };
  Ok(video.clone())
}

fn empty_video() -> Video {
  Video {
    id: VideoId {
      kind: String::from(""),
      video_id: String::from(""),
    },
    snippet: VideoSnippet {
      title: String::from(""),
      description: String::from(""),
    },
  }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
  pub region_code: String,
  pub items: Vec<Video>,
}

#[derive(Deserialize, Clone)]
pub struct Video {
  pub id: VideoId,
  pub snippet: VideoSnippet,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoId {
  pub kind: String,
  pub video_id: String,
}

#[derive(Deserialize, Clone)]
pub struct VideoSnippet {
  pub title: String,
  pub description: String,
}
