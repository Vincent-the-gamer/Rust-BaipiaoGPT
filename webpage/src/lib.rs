mod components;
mod store;

use core::messages;
use stylist::yew::styled_component;
use yew::{ Html, html };

use components::{
    header::Header,
    footer::Footer,
    title::Title
};

#[styled_component(App)]
pub fn app() -> Html {
    let msg_len = messages::len();

    html!{
        <div>
            <Header/>
            if msg_len <= 1 {  // the minimum len is 1, which is the message to tell AI to use markdown.
                <Title/>
            }
            <Footer/>
        </div>
    }
}