use vsymb::jsymb;
use yew::prelude::*;

mod button;
mod text_input;

use text_input::TextInput;

#[function_component(App)]
pub fn app() -> Html {
    let code_state = use_state(|| "".to_owned());
    let cloned_code_state: UseStateHandle<String> = code_state.clone();
    let result_state = use_state(|| "".to_owned());
    let cloned_result_state: UseStateHandle<String> = result_state.clone();

    let code_changed: Callback<String> = Callback::from(move |c: String| {
        cloned_code_state.set(c.clone());
        let ans = match c.len() {
            0 => String::new(),
            _ => match vsymb::parse_code(&c) {
                Ok(code) => format!("{}", jsymb(code)),
                Err(e) => format!("Application error: {e}"),
            },
        };
        cloned_result_state.set(ans.to_string())
    });

    html! {
        <form>
            <TextInput name="code" handle_onchange={code_changed}/>
            <p>{"Code: "}{&*code_state}</p>
            <p>{"> "}{&*result_state}</p>
        </form>
    }
}
