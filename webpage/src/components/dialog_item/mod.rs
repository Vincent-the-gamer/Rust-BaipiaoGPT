use stylist::{yew::styled_component, Style};
use web_sys::HtmlElement;
use yew::{Html, html, Properties, use_node_ref};

const CSS: &str = grass::include!("webpage/src/components/dialog_item/dialogItem.scss");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub role: String,
    pub content: String
}

#[styled_component(DialogItem)]
pub fn dialog_item(props: &Props) -> Html {
    let stylesheet = Style::new(CSS).unwrap();

    let markdown_div_ref = use_node_ref();
    let markdown_html = markdown_div_ref.cast::<HtmlElement>();

    if let Some(markdown_html) = markdown_html {
        markdown_html.set_inner_html(
            markdown::to_html(&props.content).as_str()
        );
    }

    html! {
        <div class={stylesheet}>
            if props.role == "user" || props.role == "assistant" {
                <div class="dialog-item assistant">
                    <img src="assets/imgs/avatar_assistant.jpg" alt="avatar"/>
                    <div ref={markdown_div_ref}></div>
                </div>
            } else {
                <p></p>
            }
        </div>
    }
}