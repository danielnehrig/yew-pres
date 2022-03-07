use crate::context::context::MovieContext;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/movies")]
    Movies,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn change_route(route: Route) -> yew::Callback<MouseEvent> {
    let history = use_history().unwrap();
    Callback::once(move |_| history.push(route))
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={change_route(Route::Movies)}>{ "Movies" }</button>
            <button onclick={change_route(Route::Secure)}>{ "Secure" }</button>
        </div>
    }
}

#[function_component(Movies)]
fn movies() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    let movies = use_context::<Vec<MovieContext>>().expect("no ctx found");
    if movies.len() == 0 {
        return html! {<div></div>};
    }
    html! {
        <div>
            <h1>{ "Movies" }</h1>
            {movies.iter().map(|el| html! {<div>{el.title.clone()}</div>}).collect::<Html>()}
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Movies => html! { <Movies /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let movies: UseStateHandle<Vec<MovieContext>> = use_state(|| vec![]);
    {
        let movies = movies.clone();
        use_effect_with_deps(
            move |_| {
                let movies = movies.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<MovieContext> =
                        Request::get("http://localhost:8080/tutorial/data.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    movies.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <ContextProvider<Vec<MovieContext>> context={(*movies).clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<Vec<MovieContext>>>
    }
}
