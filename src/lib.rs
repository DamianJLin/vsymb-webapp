use vsymb;
use yew::prelude::*;

mod button;
mod text_input;

use text_input::TextInput;

#[function_component(App)]
pub fn app() -> Html {
    let code_state = use_state(|| "".to_owned());
    let cloned_code_state: UseStateHandle<String> = code_state.clone();
    let j_state = use_state(|| "".to_owned());
    let cloned_j_state: UseStateHandle<String> = j_state.clone();
    let c_state = use_state(|| "".to_owned());
    let cloned_c_state: UseStateHandle<String> = c_state.clone();

    let code_changed: Callback<String> = Callback::from(move |code: String| {
        cloned_code_state.set(code.clone());

        let j_ans = match code.len() {
            0 => String::new(),
            _ => match vsymb::parse_code(&code) {
                Ok(code) => format!("{}", vsymb::jsymb(code)),
                Err(e) => format!("Application error: {e}"),
            },
        };
        cloned_j_state.set(j_ans.to_string());

        let c_ans = match code.len() {
            0 => String::new(),
            _ => match vsymb::parse_code(&code) {
                Ok(code) => format!("{}", vsymb::csymb(code)),
                Err(e) => format!("Application error: {e}"),
            },
        };
        cloned_c_state.set(c_ans.to_string());
    });

    // CSS? Wots that O_o
    let header_style = "text-align: center; font-family: Tahoma, sans-serif;";
    let style = "text-align: center; font-family: Tahoma, sans-serif;";
    let cute_box = "width: 300px; height 100px; padding: 50px; margin: auto; border: 5px outset green; background-color: lightblue; font-family: 'Courier New', monospace;";
    let grey_box =
        "width: 800px; height 100px; padding: 20px; margin: auto; border: 5px outset black;;";
    let less_margin = "margin-top: -12px;";

    html! {
        <>
            <div style={grey_box}>
                <h1 style={header_style}>{"Conway and Modified Jones Polynomial Symbol Calculator"}</h1>
                <form>
                    <p style={style}>
                        {"Enter a code for a chord diagram:"}
                    </p>
                    <p style={style}>
                        <TextInput name="code" handle_onchange={code_changed.clone()}/>
                    </p>
                </form>

                <div style={cute_box}>
                    <p>{"Code: "}{&*code_state}</p>
                    <p>{"Jones > "}{&*j_state}</p>
                    <p>{"Conway > "}{&*c_state}</p>
                </div>

                <br />

                <div style={"font-family: Tahoma, sans-serif;"}>

                    <p style={"color: black; text-align: center; font-weight: bold;"}>
                        {"NOTES"}
                    </p>
                    <p>
                        {"Valid inputs are perfect matchings: words in which every unique character appears exactly twice."}<br/>{"e.g. \"123123\" or \"P🍄Q🦀🐢PQ🍄🦀🐢\"."}
                    </p>

                    <p style={"color: red; text-align: center; font-weight: bold;"}>
                        {"IMPORTANT"}
                    </p>
                    <p>
                        {"For reasons known to the webdev gods, but not to me, this field requires you to:"}
                    </p>
                    <ul>
                        <li style={less_margin}>{"enter the code you want"}</li>
                        <li>{"click outside the field"}</li>
                        <li>{"change your input, say by deleting the last character"}</li>
                        <li>{"then click outside again"}</li>
                    </ul>
                    <p style={less_margin}>
                        {"and the first code you entered will be the one 'taken'. "}{r#"¯\_(ツ)_/¯"#}
                    </p>
                    <p>
                        {"Also, if you press enter, the page will refresh. Sorry."}
                    </p>
                    <p>
                        {"Find a CLI version of this tool (which is much more usable), at "}<a href="https://github.com/DamianJLin/vsymb">{"https://github.com/DamianJLin/vsymb"}</a>{"."}
                    </p>
                </div>

            </div>
        </>
    }
}
