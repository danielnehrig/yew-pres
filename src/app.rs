use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <main>
            <p>{*counter}</p>
            <button {onclick}>{"Click Me!"}</button>
        </main>
    }
}
