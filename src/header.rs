use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="header">
            <h1 id="name">{ "Benjamin Hinchliff" }</h1>
        </div>
    }
}
