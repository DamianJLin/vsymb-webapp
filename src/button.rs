use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(Button)]
pub fn custom_button(props: &Props) -> Html {
    html! {
        <button>{&props.label}</button>
    }
}
