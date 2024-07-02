#![recursion_limit = "256"]

use input_yew::CustomInput;
use vsymb;
use yew::prelude::*;

pub enum Msg {
    Calculate(String),
}

#[derive(Debug, Default)]
pub struct App {
    input: String,
}

impl App {
    fn get_jones_symbol_output(&self) -> String {
        match vsymb::parse_code(&self.input) {
            Ok(code) => vsymb::jsymb(code).to_string(),
            Err(e) => String::from(format!("Application error :{}", e)),
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Calculate(next_input) => self.input = next_input,
        };
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let on_change = ctx.link().callback(Msg::Calculate);
        yew::html! {
            <main>
                <div class="entry">
                    <div>
                        {"Enter a password below:"}
                        <div class="footnote">
                            {"(Will show in clear text)"}
                        </div>
                    </div>
                    <div>
                        <textarea
                            oninput=self.lin>
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {self.get_jones_symbol_output()}
                    </div>
                    <div class="footnote">
                        {"* Note: generated passwords are not actually cryptographically secure"}
                    </div>
                </div>
            </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
