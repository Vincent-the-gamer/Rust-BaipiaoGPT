use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("webpage/src/components/header/header.scss");

#[styled_component(Header)]
pub fn header() -> Html {
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <header>
                <img src="assets/imgs/logo.png" alt="logo"/>
                <h1>{"白嫖GPT"}</h1>
                <a href="https://github.com/Vincent-the-gamer/Rust-BaipiaoGPT" target="_blank">
                    <button>
                        <img src="assets/imgs/github.png" alt="github"/>
                        <span>{"GitHub"}</span>
                    </button>
                </a>
            </header>
        </div>
    }
}