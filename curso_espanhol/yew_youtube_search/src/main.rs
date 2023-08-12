use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;
use youtube::search_youtube;
mod env;
mod youtube;

fn main() {
  yew::Renderer::<RootApp>::new().render();
}

#[derive(Clone)]
struct Video {
  id: String,
  name: String,
}

#[function_component(RootApp)]
fn app() -> Html {
  let video: UseStateHandle<Option<Video>> = use_state(|| None);
  let handle_search = {
    let video = video.clone();
    Callback::from(move |text: String| {
      let video = video.clone();
      wasm_bindgen_futures::spawn_local(async move {
        match search_youtube(text).await {
          Ok(video_data) => video.set(Some(Video {
            id: video_data.id.video_id,
            name: video_data.snippet.title,
          })),
          Err(error) => {
            web_sys::console::log_1(&error.to_string().into());
          }
        };
      });
    })
  };
  // let video = Video::new(String::from("516iu_8zSIA"), String::from("nome do video"));

  let video_section = match (*video).clone() {
    Some(video) => html! {
      <VideoFrame video_id={video.id} name={video.name} />
    },
    None => html! {},
  };
  html! {
    <>
      <Search on_search={handle_search} />
      {video_section}
    </>
  }
}

#[derive(Properties, PartialEq)]
struct SearchProps {
  on_search: Callback<String>,
}
#[function_component]
fn Search(props: &SearchProps) -> Html {
  let text_to_search = use_state(|| String::new());
  let handle_input = {
    let text_to_search = text_to_search.clone();
    Callback::from(move |input_event: InputEvent| {
      let text = get_value_from_input_event(input_event);
      text_to_search.set(text)
    })
  };
  let handle_search = {
    let on_search = props.on_search.clone();
    Callback::from(move |_| {
      on_search.emit(String::from(text_to_search.to_string()));
    })
  };
  html! {
    <>
      <div>
        {"Hellow From Yew!"}
      </div>
      <div>
        <input type="text" oninput={handle_input}/>
      </div>
      <div>
        <button type="button" onclick={handle_search} >{"Buscar"}</button>
      </div>
    </>
  }
}

#[derive(Properties, PartialEq)]
struct VideoFrameProps {
  video_id: String,
  name: String,
}

#[function_component]
fn VideoFrame(props: &VideoFrameProps) -> Html {
  let video_url = format!("https://youtube.com/embed/{}", props.video_id);
  html! {
      <div>
        <h2>{&props.name}</h2>
        <iframe width="560" height="315" src={video_url} frameborder="0"/>
      </div>
  }
}

fn get_value_from_input_event(event: InputEvent) -> String {
  let event: Event = event.dyn_into().unwrap_throw();
  let event_target = event.target().unwrap_throw();
  let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
  target.value()
}
