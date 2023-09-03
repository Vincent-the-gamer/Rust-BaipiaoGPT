use serde::Deserialize;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{Html, html, Callback};
use yewdux::prelude::use_store;

use crate::store::InputContent;

const CSS: &str = grass::include!("webpage/src/components/title/title.scss");

#[derive(Deserialize)]
struct PresetQuestions{
    title: String,
    questions: Vec<String>
}

#[styled_component(Title)]
pub fn title() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    // load preset questions
    let preset_questions: Vec<PresetQuestions> = serde_json::from_str(
        include_str!("../../../utils/preset_questions.json")
    ).unwrap();

    // load textarea store
    let (_, dispatch) = use_store::<InputContent>();

    let set_question = Callback::from(move |e: MouseEvent| {
        let p = e.target().and_then(|t|{
            t.dyn_into::<HtmlElement>().ok()
        });

        if let Some(p) = p {
            (&dispatch).set(
                InputContent { 
                    text: p.inner_text()
                }
            )
        }
    });

    html!{
        <div class={stylesheet}>
            <main>
                <div class="title">
                    <h1>{"快来在线拷打GPT!!"}</h1>
                    <div class="cards">
                        {
                            preset_questions.iter().map(|item| {
                                html!{
                                    <div class="card">
                                        <h2>{ item.title.as_str() }</h2>
                                        {
                                            item.questions.iter().map(|question| {
                                                html!{
                                                    <p onclick={&set_question}>{ question }</p>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </main>
        </div>
    }
}