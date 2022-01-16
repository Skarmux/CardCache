use crate::components::{
    card::Card,
};
//use super::super::{ Anchor, AppRoute };
//use anyhow::Error;
//use yew::format::Json;
use yew::prelude::*;
//use common::*;
//use web_sys::HtmlInputElement;
//use yew::{ Component, Context, html, Html };



// #[derive(PartialEq, Properties)]
// struct Props {
    
// }

// struct State 
// {
//     games: Vec<Game>,
// }

pub struct Home
{
    //node_ref: NodeRef,
    // state: State,
    // link: ComponentLink<Self>,
    // task: Option<Referrer>,
}

// pub enum Msg
// {
//     GetGames,
// }

impl Component for Home
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self
    {
        //let games = vec![];

        Self {
            //node_ref: NodeRef::default(),
            // state: State { games },
            // task: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html
    {
        // let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <div class="tile is-ancestor is-vertical">

                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "CardVault" }</h1>
                        <h2 class="subtitle">{ "...your collections best friend!" }</h2>
                    </div>
                </div>

                // <div class="tile is-child">
                //     <figure class="image is-3by1">
                //         <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
                //     </figure>
                // </div>

                <div class="tile is-parent container">
                    { self.view_card_tiles() }
                </div>
            </div>
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool
    // {
    //     match msg {

    //         Msg::GetGames => {
    //             true
    //         }

    //     }
    // }

    // fn changed(&mut self, ctx: &Context<Self>) -> bool
    // {
    //     //self.props = props;
    //     true
    // }

    // fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    //     if first_render {
    //         if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
    //             input.focus();
    //         }
    //     }
    // }

}

impl Home
{
    fn view_card_tiles(&self) -> Html {
        //let card_id: i32 = 0;
        html! {
            <div class="columns is-multiline is-centered">
                <div class="column is-narrow"><Card card_id=0 /></div>
                <div class="column is-narrow"><Card card_id=1 /></div>
                <div class="column is-narrow"><Card card_id=2 /></div>
                <div class="column is-narrow"><Card card_id=3 /></div>
                <div class="column is-narrow"><Card card_id=4 /></div>
                <div class="column is-narrow"><Card card_id=5 /></div>
                <div class="column is-narrow"><Card card_id=6 /></div>
                <div class="column is-narrow"><Card card_id=7 /></div>
                <div class="column is-narrow"><Card card_id=8 /></div>
                <div class="column is-narrow"><Card card_id=9 /></div>
            </div>
        }
    }

    // fn view_info_tiles(&self) -> Html {
    //     html! {
    //         <>
    //             <div class="tile is-parent">
    //                 <div class="tile is-child box">
    //                     <p class="title">{ "What are yews?" }</p>
    //                     <p class="subtitle">{ "Everything you need to know!" }</p>

    //                     <div class="content">
    //                         {r#"
    //                         A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
    //                         The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
    //                         The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
    //                         but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
    //                         except on erect leading shoots where the spiral arrangement is more obvious.
    //                         The leaves are poisonous.
    //                         "#}
    //                     </div>
    //                 </div>
    //             </div>

    //             <div class="tile is-parent">
    //                 <div class="tile is-child box">
    //                     <p class="title">{ "Who are we?" }</p>

    //                     <div class="content">
    //                         { "We're a small team of just 2" }
    //                         <sup>{ 64 }</sup>
    //                         { " members working tirelessly to bring you the low-effort yew content we all desperately crave." }
    //                         <br />
    //                         {r#"
    //                             We put a ton of effort into fact-checking our posts.
    //                             Some say they read like a Wikipedia article - what a compliment!
    //                         "#}
    //                     </div>
    //                 </div>
    //             </div>
    //         </>
    //     }
    // }
}
