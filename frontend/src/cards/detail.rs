use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
//use yew::services::fetch::{ FetchService, FetchTask, Request, Response };

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub card_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    cards: Option<Vec<CardResponse>>,
    fetch_cards_task: Option<FetchTask>,
    fetch_card_task: Option<FetchTask>,
    delete_card_task: Option<FetchTask>,
}

pub enum Msg {
    MakeCardReq(i32),
    MakeCardsReq(),
    MakeDeleteCardReq(i32),
    RespCard(Result<CardResponse, anyhow::Error>),
    RespCards(Result<Vec<CardResponse>, anyhow::Error>),
    RespDeleteCard(Response<Json<Result<(), anyhow::Error>>>, i32),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeCardsReq());
        Self {
            props,
            link,
            cards: None,
            fetch_cards_task: None,
            fetch_card_task: None,
            delete_card_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                //{ self.render_detail(&self.cards)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeCardsReq() => {
                let req = Request::get(&format!("http://localhost:8000/cards"))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<CardResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespCards(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_cards_task = Some(task);
                ()
            }
            Msg::MakeCardReq(card_id) => {
                let req = Request::get(&format!("http://localhost:8000/cards/{}", card_id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<CardResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespCard(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_card_task = Some(task);
                ()
            }
            Msg::MakeDeleteCardReq(card_id) => {
                let req = Request::delete(&format!("http://localhost:8000/cards/{}", card_id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::RespDeleteCard(response, card_id)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.delete_card_task = Some(task);
                ()
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
            Msg::RespCard(_resp) => {
                // if resp.status().is_success() {
                // }
            }
            Msg::RespCards(_resp) => {
                // if resp.status().is_success() {
                // }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }
}
