use common::*;
//use super::super::{ Anchor, AppRoute };
use yew::format::{Json, Nothing};
use yew::prelude::*;
//use yew::services::fetch::{ FetchService, Referrer, Request, Response };

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Select {
    link: ComponentLink<Self>,
    games: Vec<Game>,
    fetch_games_task: Option<Referrer>,
    fetch_game_cards_task: Option<Referrer>,
}

pub enum Msg {
    MakeGamesReq(),
    RespGames(Result<Vec<GameResponse>, anyhow::Error>),
}

impl Component for Select {
    type Properties = Props;
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeGamesReq());
        Self {
            link,
            games: vec![],
            fetch_games_task: None,
            fetch_game_cards_task: None,
        }
    }

    fn view(&self) -> Html {
        let game_options: Vec<Html> = self
            .games
            .iter()
            .map(|g: &Game| {
                html! {
                    //<option value={g.id}>{g.name}</option>
                }
            })
            .collect();

        html! {
            <select>
                { game_options }
            </select>
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeGamesReq() => {
                let req = Request::get("http://localhost:8000/games/")
                    .body(Nothing)
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<GameResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespGames(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_game_cards_task = Some(task);
                ()
            }
            Msg::RespGames(resp) => {
                // if let Ok(data) = resp {
                //     self.games = Some(data);
                // }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        // self.props = props;
        true
    }
}
