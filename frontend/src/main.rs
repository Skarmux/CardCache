#![recursion_limit = "256"]

// use common::*;
//use yew::{ Component, Context, html, Html };
use yew::prelude::*;
use yew_router::prelude::*;
//use wasm_bindgen::prelude::*;
//use web_sys::HtmlInputElement;

mod components;
mod pages;
//mod games;
//mod cards;

use pages::{
    home::Home, page_not_found::PageNotFound,
};
//use yew::html::Scope;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route
{
    #[at("/game/create")]
    GameCreate,

    #[at("/game/:id")]
    GameDetail { id: u32 },

    #[at("/card/create")]
    CardCreate,

    #[at("/card/:id")]
    CardDetail { id: u32 },

    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

// struct State {
//     games: Vec<Game>,
//     cards: Vec<Card>,
//     illustrators: Vec<Illustrator>
// }

pub enum Msg
{
    // AddToCollection()
}

struct App
{
    //state: State,
    //link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = (); // root component can't get properties passed by parent

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // state: State {
            //     games: vec![],
            //     cards: vec![],
            //     illustrators: vec![],
            // },
            // link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    // fn changed(&mut self, _ctx: &Context<Self>) -> bool {
    //     true
    // }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        //let games = self.state.games.clone();
        //let illustrators = self.state.illustrators.clone();

        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </BrowserRouter>
            // <div class={classes!("app")}>
            //     <div class={classes!("nav")}>
            //         <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
            //             { "Home" }
            //         </Link<Route>>
            //     </div>
            //     <div class={classes!("content")}>

            //         {"Hello, World!"}

            //         <Link<Route> to={Route::GameCreate}>
            //             { "GameCreate" }
            //         </Link<Route>>

            //         <Link<Route> to={Route::GameDetail { id: 0 } }>
            //             { "GameDetail" }
            //         </Link<Route>>

            //         <Link<Route> to={Route::CardCreate}>
            //             { "CardCreate" }
            //         </Link<Route>>

            //         <Link<Route> to={Route::CardDetail { id: 0 } }>
            //             { "CardDetail" }
            //         </Link<Route>>

            //     </div>
            // </div>
        }
    }
}

impl App
{

}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::GameCreate => {
            html! { 
                //<PostList /> 
            }
        }
        Route::GameDetail { id: _ } => {
            html! { 
                //<Author seed={id} /> 
            }
        }
        Route::CardCreate => {
            html! { 
                //<AuthorList /> 
            }
        }
        Route::CardDetail { id: _ } => {
            html! { 
                //<Home /> 
            }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

//#[wasm_bindgen(start)]
fn main() {
    //App::<App>::new().mount_to_body();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
