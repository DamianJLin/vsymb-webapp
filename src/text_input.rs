use gloo::console::log;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange: Callback<String> = props.handle_onchange.clone();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change: Callback<Event> = Callback::from(move |e: Event| {
        log!(input_value.clone());
        let target: EventTarget = e.target().unwrap();
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
        handle_onchange.emit(input_value.clone());
    });

    html! {
        <input type="text" name={props.name.clone()} onchange={on_change} />
    }
}
