mod components;
mod store;

use stylist::{yew::styled_component};
use yew::{ Html, html };

use components::{
    header::Header,
    footer::Footer
};

#[styled_component(App)]
pub fn app() -> Html {
    html!{
        <div>
            <Header/>
            <Footer/>
        </div>
    }
}