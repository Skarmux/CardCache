//use crate::Route;
//use common::*;
use yew::prelude::*;
//use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props
{
    pub card_id: i32,
}

pub struct Card
{
    id: i32,
}

impl Component for Card
{
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            //author: Author::generate_from_seed(ctx.props().seed),
            id: ctx.props().card_id,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        //self.author = Author::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { id: _ } = self;
        html! {
            <div class="w-40">
                // <p class="text-center font-bold">{ format!{"Name {}", &id} }</p>
                <figure>
                    <img alt="Cracked Bauble" src={"https://storage.googleapis.com/fabmaster/media/images/U-WTR224.width-450.png"} />
                </figure>
                // <p class="text-center">{ format!{"{},99€", &id} }</p>
            </div>
        }
    }
}