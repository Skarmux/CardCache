use common::*;
use serde_json::from_str;
use yew_router::{ agent::{ RouteAgent, RouteRequest }, prelude::* };
use yew::format::Json;
use yew::prelude::*;
//use yew::services::{ fetch::{ FetchService, FetchTask, Request, Response }, ConsoleService };

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    //pub games: Vec<Game>,
    //pub illustrators: Vec<Illustrator>
}

pub struct Create
{
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_name: String,
    state_code: String,
    state_game_id: Option<i32>,
    state_illustrator_id: Option<i32>,
}

pub enum Msg
{
    MakeReq(),
    Resp(Result<CardResponse, anyhow::Error>),

    EditName(String),
    EditCode(String),
    EditGame(String),
    EditIllustrator(String),
}

impl Create
{
    fn render_form(&self) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));

        let edit_code = self
            .link
            .callback(move |e: InputData| Msg::EditCode(e.value));

        let edit_game = self.link.callback(move |e: ChangeData| 
            match e {
                ChangeData::Select(elem) => Msg::EditGame(elem.value()),
                _ => unreachable!("only used on select field"),
            }
        );

        let edit_illustrator = self.link.callback(move |e: ChangeData| 
            match e {
                ChangeData::Select(elem) => Msg::EditIllustrator(elem.value()),
                _ => unreachable!("only used on select field"),
            }
        );

        // let game_options: Vec<Html> = self.props.games
        //     .iter()
        //     .map(|g: &Game| {
        //         html! {
        //             //<option value={g.id}>{g.name}</option>
        //         }
        //     })
        //     .collect();

        // let illustrator_options: Vec<Html> = self.props.illustrators
        //     .iter()
        //     .map(|i: &Illustrator| {
        //         html! {
        //             //<option value={i.id}>{i.name}</option>
        //         }
        //     })
        //     .collect();

        html! {
            <div class={classes!("card-form")}>
                <div>
                    {"Game:"}
                    <select onchange={edit_game}>
                        //{game_options}
                    </select>
                    <br/>
                    {"Name:"}
                    <input type="text" value={self.state_name.clone()} oninput={edit_name} />
                    <br/>
                    {"Code:"}
                    <input type="text" value={self.state_code.clone()} oninput={edit_code} />
                    <br/>
                    {"Illustrator:"}
                    <select onchange={edit_illustrator}>
                        <option value="" selected=true>{"None"}</option>
                        //{illustrator_options}
                    </select>
                </div>
                <div>
                    <button onclick={self.link.callback(move |_| Msg::MakeReq())}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

impl Component for Create
{
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            fetch_task: None,
            state_name: String::new(),
            state_code: String::new(),
            state_game_id: None,
            state_illustrator_id: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            {self.render_form()}
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {

            Msg::MakeReq() => {
                let body = CardRequest {
                    name: self.state_name.clone(),
                    code: self.state_code.clone(),
                    game_id: self.state_game_id.unwrap(),
                    illustrator_id: self.state_illustrator_id,
                };

                let req = Request::post(&format!("http://localhost:8000/cards"))
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<CardResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }

            Msg::Resp(resp) => {
                ConsoleService::info(&format!("card created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: format!("/games/{}", self.state_game_id.unwrap()),
                        state: (),
                    }));
                }
            }

            Msg::EditName(input) => {
                self.state_name = input;
            }

            Msg::EditCode(input) => {
                self.state_code = input;
            }

            Msg::EditGame(input) => {
                ConsoleService::info(&format!("input: {:?}", input));
                let res = from_str::<i32>(&input);
                self.state_game_id = res.ok();
            }

            Msg::EditIllustrator(input) => {
                ConsoleService::info(&format!("input: {:?}", input));
                let res = from_str::<i32>(&input);
                self.state_illustrator_id = res.ok();
            }

        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        //self.props = props;
        true
    }
}
