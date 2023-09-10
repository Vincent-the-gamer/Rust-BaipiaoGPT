use core::{messages, services::chat};

use gloo_console::log;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlTextAreaElement;
use yew::{Html, html, Callback, use_node_ref};
use yewdux::prelude::use_store;

use crate::store::{InputContent, DialogStore};

const CSS: &str = grass::include!("webpage/src/components/footer/footer.scss");

#[styled_component(Footer)]
pub fn footer() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    let textarea_ref = use_node_ref();

    // textarea height auto fit
    // let input_change_style: Callback<web_sys::InputEvent> = {
    //     let textarea_ref = textarea_ref.clone();

    //     Callback::from(move |_| {
    //         let text_area = textarea_ref.cast::<HtmlTextAreaElement>();
    
    //         if let Some(text_area) = text_area {
    //             let scroll_height = text_area.scroll_height().to_string() + "px";
    //             text_area.style().set_property("height", "auto").unwrap();
    //             text_area.style().set_property(
    //                 "height", 
    //                 scroll_height.as_str()
    //             ).unwrap();
    //         }
    //     })
    // };


    // sync store to textarea
    let (store_state, input_dispatch) = use_store::<InputContent>();
    let (_, dialog_dispatch) = use_store::<DialogStore>();
    
    // textarea oninput
    let textarea_input = {
        let textarea_ref = textarea_ref.clone();
        let input_dispatch = input_dispatch.clone();

        Callback::from(move |_| {
            let text_area = textarea_ref.cast::<HtmlTextAreaElement>();
    
            if let Some(text_area) = text_area {
                input_dispatch.set(
                    InputContent { 
                        text: text_area.value()
                    }
                );
            }
        })
    };

    // send message
    let send = {
        let state = store_state.clone();
        let input_dispatch = input_dispatch.clone();
        let dialog_dispatch = dialog_dispatch.clone();

        if state.text.clone() != String::from(""){
            dialog_dispatch.reduce_mut_callback(move |dialog| {
                dialog.messages = messages::get_messages();
                let text = state.text.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = chat(text.as_str()).await.unwrap();
                    log!(result);
                });

                input_dispatch.set(
                    InputContent{
                        text: String::from("")
                    }
                );

                dialog.len = messages::len();
            })
        } else {
            Callback::from(|_| {})
        }
    };

    
    html!{
        <div class={stylesheet}>
            <footer>
                <div class="input-area">
                    <textarea ref={textarea_ref} 
                              oninput={textarea_input}
                              placeholder="输入问题，拷打GPT!"
                              value={ store_state.text.clone() }
                    />
                    <button class="send"
                            disabled={ store_state.text.clone() == String::from("") }
                            onclick={ send }>
                            {"发送"}
                    </button>
                </div>
            </footer>
        </div>
    }
}