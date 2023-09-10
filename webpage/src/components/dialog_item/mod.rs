use stylist::{yew::styled_component, Style};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;
use yew::{Html, html, Properties, use_node_ref};

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

    let markdown_div_ref = use_node_ref();
    let markdown_html = markdown_div_ref.cast::<HtmlElement>();

    if let Some(markdown_html) = markdown_html {
        markdown_html.set_inner_html(
            markdown::to_html(&props.content)
                     .as_str()
        );
        // 调用hljs.highlightAll()
        highlight_all();
    }

    html! {
        <div class={stylesheet}>
            if props.role == "user" {
                <div class="dialog-item">
                    <img src="assets/imgs/avatar_user.jpg" alt="avatar"/>
                    <h3>{&props.content}</h3>
                </div>
            } else if props.role == "assistant" {
                <div class="dialog-item">
                    <img src="assets/imgs/avatar_assistant.jpg" alt="avatar"/>
                    <div ref={markdown_div_ref}></div>
                </div>
            } else {
                <p></p>
            }
        </div>
    }
}