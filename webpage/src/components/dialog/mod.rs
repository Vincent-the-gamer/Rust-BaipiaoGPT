use core::messages;

use stylist::{yew::styled_component, Style};
use yew::{Html, html};
use yewdux::prelude::use_store;

use crate::{
    store::DialogStore, 
    components::dialog_item::DialogItem
};

const CSS: &str = grass::include!("webpage/src/components/dialog/dialog.scss");

#[styled_component(Dialog)]
pub fn dialog() -> Html{
    let stylesheet = Style::new(CSS).unwrap();
    let (_, dialog_dispatch) = use_store::<DialogStore>();

    // clear context
    let clear_context = dialog_dispatch.reduce_mut_callback(|dialog| {
        messages::clear_message();
        dialog.len = messages::len();
    });

    let msg_list = messages::get_messages();


    html!{
        <div class={stylesheet}>
            <main>
                <button class="clear" onclick={ clear_context }>{"清空对话"}</button>
                <div class="dialog-area">
                    {
                        msg_list[1..].iter().map(|msg|{
                            html!{
                                <DialogItem role={msg.role.to_owned()} content={msg.content.to_owned()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </main>
        </div>
    }
}