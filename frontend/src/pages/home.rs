use crate::components::card::Card;
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

pub struct Home {
    //node_ref: NodeRef,
    // state: State,
    // link: ComponentLink<Self>,
    // task: Option<Referrer>,
}

// pub enum Msg
// {
//     GetGames,
// }

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        //let games = vec![];

        Self {
            //node_ref: NodeRef::default(),
            // state: State { games },
            // task: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let onclick = ctx.link().callback(|_| Msg::Click);
        html! {
            <div class="flex flex-col gap-4 text-white">
                <h1 class="text-lg font-bold">{"Welcome To Rathe"}</h1>
                { self.view_card_tiles() }
                <h1 class="text-lg font-bold">{"Arcane Rising"}</h1>
                { self.view_card_tiles() }
                <h1 class="text-lg font-bold">{"Crucible Of War"}</h1>
                { self.view_card_tiles() }
                <h1 class="text-lg font-bold">{"Monarch"}</h1>
                { self.view_card_tiles() }
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

impl Home {
    fn view_card_tiles(&self) -> Html {
        //let card_id: i32 = 0;
        html! {
            <div class="grid grid-cols-6 place-items-center gap-8 space-x-1">
                <Card card_id=0 />
                <Card card_id=1 />
                <Card card_id=2 />
                <Card card_id=3 />
                <Card card_id=4 />
                <Card card_id=5 />
                <Card card_id=6 />
                <Card card_id=7 />
                <Card card_id=8 />
                <Card card_id=9 />
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
