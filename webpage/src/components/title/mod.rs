use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("webpage/src/components/title/title.scss");

#[styled_component(Title)]
pub fn title() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <main>
                <h1>{"啊啊啊"}</h1>
            </main>
        </div>
    }
}