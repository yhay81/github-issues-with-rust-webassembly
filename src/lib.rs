extern crate failure;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

extern crate comrak;
use comrak::{markdown_to_html, ComrakOptions};

pub struct Model {
    fetching: bool,
    data: Vec<Issue>,
    ft: Option<FetchTask>,
    owner: String,
    repo: String,
}

pub enum Msg {
    FetchData(),
    FetchReady(Result<Vec<Issue>, Error>),
    UpdateOwner(String),
    UpdateRepo(String),
    Ignore,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    title: String,
    body: String,
    state: String,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<FetchService> + 'static,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fetching: false,
            data: vec![],
            ft: None,
            owner: "".into(),
            repo: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData() => {
                self.fetching = true;
                let task = {
                    let callback = env.send_back(
                        move |response: Response<Json<Result<Vec<Issue>, Error>>>| {
                            let (meta, Json(issues)) = response.into_parts();
                            if meta.status.is_success() {
                                Msg::FetchReady(issues)
                            } else {
                                Msg::Ignore // FIXME: Handle this error accordingly.
                            }
                        },
                    );
                    let request = Request::get(format!(
                        "https://api.github.com/repos/{}/{}/issues",
                        self.owner, self.repo
                    )).body(Nothing)
                        .unwrap();
                    let fetch_service: &mut FetchService = env.as_mut();
                    fetch_service.fetch(request, callback)
                };
                self.ft = Some(task)
            }
            Msg::FetchReady(issues) => {
                self.fetching = false;
                // self.data = issues.map(|data| data.id).ok()
                self.data = issues.unwrap()
            }
            Msg::UpdateOwner(val) => {
                self.owner = val;
            }
            Msg::UpdateRepo(val) => {
                self.repo = val;
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<FetchService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
            <header>
            <div class="header-center",>
                <h1 class="head-title",>{"GitHub Issues"}</h1>
                <div class="head-right",>
                    <input type="text", value=&self.owner, oninput=|e: InputData| Msg::UpdateOwner(e.value), />
                    <span>{" / "}</span>
                    <input type="text", value=&self.repo, oninput=|e: InputData| Msg::UpdateRepo(e.value), />
                    <button class="head-button", onclick=|_| Msg::FetchData(),>{ "Fetch Issues" }</button>
                </div>
            </div>
            </header>

            <main>
            { self.view_data() }
            </main>
            </div>
        }
    }
}

impl Model {
    fn view_data<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + 'static,
    {
        let view_issue = |issue: &Issue| {
            html! {
                <div class="post", >
                    <h2>{ &issue.title }</h2>
                    // <div class="issue-content",>{ &issue.body }</div>
                    <div class="issue-content",>{ markdown_to_html(&issue.body, &ComrakOptions::default()) }</div>
                    <div class="issue-state",>{ &issue.state }</div>
                </div>
            }
        };
        html! {
            <div class="issues", >
            { for self.data.iter().map(view_issue) }
            </div>
        }
    }
}
