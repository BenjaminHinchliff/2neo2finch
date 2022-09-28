use yew::prelude::*;

#[function_component(Loader)]
pub fn loader() -> Html {
    html! {
        <div class="lds-heart"><div></div></div>
    }
}
