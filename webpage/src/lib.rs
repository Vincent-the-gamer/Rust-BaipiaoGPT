mod components;
mod store;

use core::messages;
use stylist::yew::styled_component;
use yew::{ Html, html, use_effect };

use components::{
    header::Header,
    footer::Footer,
    title::Title,
    dialog::Dialog
};

use yewdux::prelude::use_store;
use crate::store::DialogStore;

#[styled_component(App)]
pub fn app() -> Html {
    let (dialog_state, dialog_dispatch) = use_store::<DialogStore>();
    let dialog_len = dialog_state.len;

    use_effect(move || {
        // onMounted
        dialog_dispatch.set(
            DialogStore {
                len: messages::len()
            }
        );

        // onBeforeUnmount
        // || messages::clear_message()
    });

    html!{
        <div>
            <Header/>
            if dialog_len <= 1 {  // the minimum len is 1, which is the message to tell AI to use markdown.
                <Title/>
            } else {
                <Dialog/>
            }
            <Footer/>
        </div>
    }
}