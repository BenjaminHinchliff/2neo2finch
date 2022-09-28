use crate::loader::Loader;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct RepoInfoProps {
    pub repo: Repo,
}

#[function_component(RepoInfo)]
pub fn repo_info(RepoInfoProps { repo }: &RepoInfoProps) -> Html {
    html! {
        <div class="repo">
            <a href={repo.html_url.clone()} target="_blank">
                <h2 class="repo-name">{ &repo.name }</h2>
                <p class="repo-desc">{ repo.description.as_deref().unwrap_or("(no description)") }</p>
            </a>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct RepoListProps {
    pub repos: Vec<Repo>,
}

#[function_component(RepoList)]
pub fn repo_list(RepoListProps { repos }: &RepoListProps) -> Html {
    html! {
        <div>
            if repos.is_empty() {
                <Loader />
            } else {
                { for repos.iter().map(|repo| {
                    html! {
                        <RepoInfo repo={repo.clone()} />
                    }
                }) }
            }
        </div>
    }
}
