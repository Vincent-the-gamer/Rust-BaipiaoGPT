use stylist::{yew::styled_component, Style};
use web_sys::HtmlTextAreaElement;
use yew::{Html, html, Callback, use_node_ref};

const CSS: &str = grass::include!("webpage/src/components/footer/footer.scss");

#[styled_component(Footer)]
pub fn footer() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    let textarea_ref = use_node_ref();

    // textarea height auto fit
    let input_change_style: Callback<web_sys::InputEvent> = {
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |_| {
            let text_area = textarea_ref.cast::<HtmlTextAreaElement>();
    
            if let Some(text_area) = text_area {
                let scroll_height = text_area.scroll_height().to_string() + "px";
                text_area.style().set_property("height", "auto").unwrap();
                text_area.style().set_property(
                    "height", 
                    scroll_height.as_str()
                ).unwrap();
            }
        })
    };

    
    html!{
        <div class={stylesheet}>
            <footer>
                <div class="input-area">
                    <textarea ref={textarea_ref} 
                              oninput={input_change_style}
                              placeholder="输入问题，拷打GPT!"/>
                    <button class="send">{"发送"}</button>
                </div>
            </footer>
        </div>
    }
}