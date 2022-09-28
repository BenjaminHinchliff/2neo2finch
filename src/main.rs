use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <div class="header">
                <h1 id="name">{ "Benjamin Hinchliff" }</h1>
                // <img src="img/logo.svg" alt="Site logo of a tessalated cat" />
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
