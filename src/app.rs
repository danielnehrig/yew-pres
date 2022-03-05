use std::sync::{Arc, Mutex};

use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let videos = use_state(|| vec![]);
    let a_video = Arc::new(Mutex::new(videos.clone()));
    {
        let a_video = Arc::clone(&a_video);
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let videos = a_video.lock().unwrap();
                    let fetched_videos: Vec<Video> =
                        Request::get("http://localhost:8080/tutorial/data.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <main>
            <ul>
            <li>{"test"}</li>
            </ul>
            {if videos.len() > 0 { videos.clone()[1].title.to_string()} else {"No Videos".to_string()}}
        </main>
    }
}
