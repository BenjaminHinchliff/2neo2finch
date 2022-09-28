mod header;
mod repository;
mod loader;

use gloo_net::http::Request;
use header::Header;
use repository::{Repo, RepoList};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let repos = use_state(Vec::new);
    {
        let repos = repos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_repos: Vec<Repo> =
                        Request::get("https://api.github.com/users/BenjaminHinchliff/repos")
                            .query([("sort", "pushed")])
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    repos.set(fetched_repos);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="container">
            <Header />
            <RepoList repos={(*repos).clone()} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
