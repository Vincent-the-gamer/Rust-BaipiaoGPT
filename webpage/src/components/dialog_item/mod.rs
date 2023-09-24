use stylist::{yew::styled_component, Style};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{Html, html, Properties, AttrValue};

const CSS: &str = grass::include!("webpage/src/components/dialog_item/dialogItem.scss");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub role: String,
    pub content: String
}

// 访问外部hljs.highlightAll()
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll)]
    fn highlight_all();
}

#[styled_component(DialogItem)]
pub fn dialog_item(props: &Props) -> Html {
    let stylesheet = Style::new(CSS).unwrap();
    highlight_all();

    html! {
        <div class={stylesheet}>
            if props.role == String::from("user") || props.role == "user" {
                <div class="dialog-item">
                    <img src="assets/imgs/avatar_user.jpg" alt="avatar"/>
                    <h3>{ &props.content }</h3>
                </div>
            } else if props.role == String::from("assistant") || props.role == "assistant" {
                <div class="dialog-item">
                    <img src="assets/imgs/avatar_assistant.jpg" alt="avatar"/>
                    {          
                        Html::from_html_unchecked(
                            AttrValue::from(
                                markdown::to_html(props.content.as_str())
                            )
                        )
                    }
                </div>
            } else {
                <p></p>
            }
        </div>
    }
}