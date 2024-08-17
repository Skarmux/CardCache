use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
//use yew::services::fetch::{ , Referrer, Request, Response };

pub struct List {
    fetch_task: Option<Referrer>,
    games: Option<Vec<GameResponse>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    MakeReq(),
    Resp(Result<Vec<GameResponse>, anyhow::Error>),
}

impl Component for List {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq());
        Self {
            fetch_task: None,
            games: None,
            link,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_list() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq() => {
                self.games = None;
                let req = Request::get("http://localhost:8000/games")
                    .body(Nothing)
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<GameResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.games = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }
}

impl List {
    fn render_list(&self) -> Html {
        if let Some(t) = &self.games {
            html! {
                <div class={classes!("list")}>
                    { t.iter().map(|name| self.view_game(name)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div class={classes!("loading")}>{"loading..."}</div>
            }
        }
    }

    fn view_game(&self, game: &GameResponse) -> Html {
        html! {
            <div class={classes!("list-item")}>
                <Anchor route={AppRoute::GameDetail(game.id as i32)}>
                    { &game.name }
                </Anchor>
            </div>
        }
    }
}
