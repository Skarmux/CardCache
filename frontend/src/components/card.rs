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
        let Self { id } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img alt="Cracked Bauble" src={"https://storage.googleapis.com/fabmaster/media/images/U-WTR224.width-450.png"} />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &id }</p>
                            <p>
                                { "Info Text" }
                            </p>
                        </div>
                    </div>
                </div>
                // <footer class="card-footer">
                //     <Link<Route> classes={classes!("card-footer-item")} to={Route::Author { id: author.seed }}>
                //         { "Profile" }
                //     </Link<Route>>
                // </footer>
            </div>
        }
    }
}