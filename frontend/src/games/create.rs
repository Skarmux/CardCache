use common::*;
use yew::format::Json;
use yew::prelude::*;
//use yew::services::{ fetch::{ FetchService, FetchTask, Request, Response}, ConsoleService };
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
};

pub struct Create {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_name: String,
    state_acronym: String,
}

pub enum Msg {
    MakeReq(),
    Resp(Result<GameResponse, anyhow::Error>),
    EditName(String),
    EditAcronym(String),
}

impl Create {
    fn render_form(&self) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));
        let edit_acronym = self
            .link
            .callback(move |e: InputData| Msg::EditAcronym(e.value));

        html! {
            <div class={classes!("game-form")}>
                <div>
                    { "Name" }
                    <input type="text" value={self.state_name.clone()} oninput={edit_name} />
                    <br/>
                    { "Kürzel" }
                    <input type="text" value={self.state_acronym.clone()} oninput={edit_acronym} />
                </div>
                <div>
                    <button onclick={self.link.callback(move |_| Msg::MakeReq())}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

impl Component for Create {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            state_name: String::new(),
            state_acronym: String::new(),
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq() => {
                let body = GameRequest {
                    name: self.state_name.clone(),
                    acronym: self.state_acronym.clone(),
                };
                let req = Request::post("http://localhost:8000/games")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<GameResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("game created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: "/".to_string(),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                self.state_name = input;
            }
            Msg::EditAcronym(input) => {
                self.state_acronym = input;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }
}
