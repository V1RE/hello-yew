use yew::{function_component, html, use_state, Callback};

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
    <div>
        <button {onclick}>{"Add one"}</button>
        <p>
            <b>{ "Current value: " }</b>
            { *counter }
        </p>
    </div>
    }
}

fn main() {
    yew::start_app::<Counter>();
}
