use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
//use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub game_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    cards: Option<Vec<CardResponse>>,
    game: Option<GameResponse>,
    fetch_game_cards_task: Option<FetchTask>,
    fetch_game_task: Option<FetchTask>,
    delete_card_task: Option<FetchTask>,
}

pub enum Msg {
    MakeCardsReq(i32),
    RespCards(Result<Vec<CardResponse>, anyhow::Error>),
    MakeGameReq(i32),
    RespGame(Result<GameResponse, anyhow::Error>),
    MakeDeleteCardReq(i32),
    RespDeleteCard(Response<Json<Result<(), anyhow::Error>>>, i32),
}

impl Detail {
    fn render_detail(
        &self,
        game: &Option<GameResponse>,
        cards: &Option<Vec<CardResponse>>,
    ) -> Html {
        match game {
            Some(g) => {
                html! {
                    <div class={classes!("detail")}>
                        <h1>{&g.name}{" (ID:"}<span class={classes!("id")}>{g.id}</span>{")"}</h1>
                        { self.view_card_list(cards) }
                        <br/>
                        <Anchor route={AppRoute::CardCreate()}>{"Create New Card"}</Anchor>
                    </div>
                }
            }
            None => {
                html! {
                    <div class={classes!("loading")}>{"loading..."}</div>
                }
            }
        }
    }

    fn view_card_list(&self, cards: &Option<Vec<CardResponse>>) -> Html {
        match cards {
            Some(c) => {
                html! {
                    c.iter().map(|card| self.view_card(card)).collect::<Html>()
                }
            }
            None => {
                html! {
                    <div class={classes!("loading")}>{"loading..."}</div>
                }
            }
        }
    }

    fn view_card(&self, card: &CardResponse) -> Html {
        let card_id = card.id;
        html! {
            <div class={classes!("list-item", "card")}>
                <div><b>{ &card.name }</b>{ " (" }<button onclick={self.link.callback(move |_| Msg::MakeDeleteCardReq(card_id))}>{"Delete"}</button>{")"}</div>
                <div>{ &card.code }</div>
            </div>
        }
    }
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeGameReq(props.game_id));
        link.send_message(Msg::MakeCardsReq(props.game_id));
        Self {
            props,
            link,
            game: None,
            cards: None,
            fetch_game_cards_task: None,
            fetch_game_task: None,
            delete_card_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.game, &self.cards) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeCardsReq(game_id) => {
                let req = Request::get(&format!("http://localhost:8000/games/{}/cards", game_id))
                    .body(Nothing)
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<CardResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespCards(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_game_cards_task = Some(task);
                ()
            }
            Msg::MakeGameReq(game_id) => {
                let req = Request::get(&format!("http://localhost:8000/games/{}", game_id))
                    .body(Nothing)
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<GameResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespGame(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_game_task = Some(task);
                ()
            }
            Msg::MakeDeleteCardReq(card_id) => {
                let req = Request::delete(&format!("http://localhost:8000/cards/{}", card_id))
                    .body(Nothing)
                    .expect("can make request to backend");

                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::RespDeleteCard(response, card_id)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.delete_card_task = Some(task);
                ()
            }
            Msg::RespGame(resp) => {
                if let Ok(data) = resp {
                    self.game = Some(data);
                }
            }
            Msg::RespCards(resp) => {
                if let Ok(data) = resp {
                    self.cards = Some(data);
                }
            }
            Msg::RespDeleteCard(resp, card_id) => {
                if resp.status().is_success() {
                    self.cards = self.cards.as_ref().map(|cards| {
                        cards
                            .into_iter()
                            .filter(|p| p.id != card_id)
                            .cloned()
                            .collect()
                    });
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }
}
