use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={ctx.clone()}>
            <main>
                <p style={format!("color: {}", (*ctx).foreground)}>{*counter}</p>
                <button {onclick}>{"Click Me!"}</button>
                <ChangeColor />
            </main>
        </ContextProvider<UseStateHandle<Theme>>>
    }
}

#[function_component(ChangeColor)]
pub fn change_color() -> Html {
    let theme = use_context::<UseStateHandle<Theme>>().expect("Not Ctx found");
    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(Theme {
                foreground: "#3dff3d".to_owned(),
                background: theme.background.clone(),
            });
        })
    };

    html! {
        <div>
            <button {onclick}>{format!("Current Color {}", (*theme).foreground)}</button>
        </div>
    }
}
