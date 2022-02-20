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
use yew::html::Scope;

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

    fn view(&self, ctx: &Context<Self>) -> Html {

        //let games = self.state.games.clone();
        //let illustrators = self.state.illustrators.clone();

        html! {
            <BrowserRouter>
                <div class="relative flex">
                    // navbar
                    { self.view_nav(ctx.link())}
                    // content
                    <div class="flex-1 ml-16 p-4 bg-gray-800">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            </BrowserRouter>
        }
    }
}

impl App
{
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="fixed top-0 left-0 h-screen w-16 p-2 gap-1
                        justify-items-centered
                        flex flex-col
                      bg-gray-900 text-white shadow-lg">
                <svg class="fill-current sidebar-icon" xmlns="http://www.w3.org/2000/svg" viewBox="-230 0 1024 512">
                    <path d="M575.8 255.5C575.8 273.5 560.8 287.6 543.8 287.6H511.8L512.5 447.7C512.5 450.5 512.3 453.1 512 455.8V472C512 494.1 494.1 512 472 512H456C454.9 512 453.8 511.1 452.7 511.9C451.3 511.1 449.9 512 448.5 512H392C369.9 512 352 494.1 352 472V384C352 366.3 337.7 352 320 352H256C238.3 352 224 366.3 224 384V472C224 494.1 206.1 512 184 512H128.1C126.6 512 125.1 511.9 123.6 511.8C122.4 511.9 121.2 512 120 512H104C81.91 512 64 494.1 64 472V360C64 359.1 64.03 358.1 64.09 357.2V287.6H32.05C14.02 287.6 0 273.5 0 255.5C0 246.5 3.004 238.5 10.01 231.5L266.4 8.016C273.4 1.002 281.4 0 288.4 0C295.4 0 303.4 2.004 309.5 7.014L564.8 231.5C572.8 238.5 576.9 246.5 575.8 255.5L575.8 255.5z"/>
                </svg>
                <svg class="fill-current sidebar-icon" xmlns="http://www.w3.org/2000/svg" viewBox="-230 0 1024 512">
                    <path d="M144 240C144 195.8 179.8 160 224 160C268.2 160 304 195.8 304 240C304 284.2 268.2 320 224 320C179.8 320 144 284.2 144 240zM512 0C547.3 0 576 28.65 576 64V416C576 451.3 547.3 480 512 480H496L480 512H416L400 480H176L160 512H96L80 480H64C28.65 480 0 451.3 0 416V64C0 28.65 28.65 0 64 0H512zM224 400C312.4 400 384 328.4 384 240C384 151.6 312.4 80 224 80C135.6 80 64 151.6 64 240C64 328.4 135.6 400 224 400zM480 221.3C498.6 214.7 512 196.9 512 176C512 149.5 490.5 128 464 128C437.5 128 416 149.5 416 176C416 196.9 429.4 214.7 448 221.3V336C448 344.8 455.2 352 464 352C472.8 352 480 344.8 480 336V221.3z"/>
                </svg>
            </nav>
        }
    }
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
