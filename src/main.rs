mod header;
mod loader;
mod repository;

use gloo_net::http::Request;
use header::Header;
use repository::RepoList;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let repos = use_state(|| Ok(Vec::new()));
    {
        let repos = repos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let repos_req =
                        Request::get("https://api.github.com/users/BenjaminHinchliff/repos")
                            .query([("sort", "pushed")])
                            .send()
                            .await;

                    // TODO: find a less messy way of doing this in awaited code
                    let repos_req = match repos_req {
                        Ok(req) => req,
                        Err(err) => {
                            repos.set(Err(err));
                            return;
                        }
                    };

                    let fetched_repos = repos_req.json().await;

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
            if let Ok(repos) = &*repos {
                <RepoList repos={repos.clone()} />
            } else {
                <p class="error">{ "failed to fetch repos - maybe try again later?" }</p>
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
