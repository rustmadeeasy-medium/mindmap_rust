use yew::prelude::*;
use std::collections::HashMap;
// Props for the KeywordInput component
#[derive(Properties, PartialEq)]
pub struct KeywordInputProps {
    pub key_name: String,
    pub value: String,
    pub on_update: Callback<(String, String)>,
}

// KeywordInput Component
#[function_component(KeywordInput)]
pub fn keyword_input(props: &KeywordInputProps) -> Html {
    let key_name = props.key_name.clone();
    let value = props.value.clone();
    let on_update = props.on_update.clone();

    html! {
        <div style="margin-bottom: 10px;">
            <label for={key_name.clone()}>{format!("{}:", key_name)}</label>
            <input
                id={key_name.clone()}
                value={value.clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                        on_update.emit((key_name.clone(), input.value()));
                    }
                })}
                style="margin-left: 10px;"
            />
        </div>
    }
}

// Props for the Snippet component
#[derive(Properties, PartialEq)]
pub struct SnippetProps {
    pub snippet: String,
}

// Snippet Component
#[function_component(Snippet)]
pub fn snippet(props: &SnippetProps) -> Html {
    html! {
        <pre
            style="
                background-color: #f4f4f4;
                padding: 10px;
                border-radius: 5px;
                font-family: monospace;
                margin-top: 20px;
            "
        >
            { &props.snippet }
        </pre>
    }
}

// Props for the Ownership component
#[derive(Properties, PartialEq)]
pub struct OwnershipProps {
    #[prop_or_default]
    pub on_close: Callback<MouseEvent>,
}

// Ownership Component
#[function_component(Ownership)]
pub fn ownership(props: &OwnershipProps) -> Html {
    let keywords = use_state(|| {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("var_name"), "x".to_string());
        map.insert(String::from("value"), "10".to_string());
        map
    });

    let update_keyword = {
        let keywords = keywords.clone();
        Callback::from(move |(key, value): (String, String)| {
            let mut updated_keywords: HashMap<String, String> = (*keywords).clone();
            updated_keywords.insert(key.clone(), value);
            keywords.set(updated_keywords);
        })
    };

    let snippet = {
        let keywords = (*keywords).clone();
        format!(
            "let {} = {};\nprintln!(\"Value: {}\", {});",
            keywords["var_name"], keywords["value"], keywords["var_name"], keywords["var_name"]
        )
    };

    html! {
        <div
            style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                background-color: rgba(255, 255, 255, 0.9); /* Semi-transparent background */
                border: 1px solid black;
                box-sizing: border-box;
                z-index: 9999;
            "
        >
            <h3 style="margin-bottom: 20px;">{ "Editable Code Snippet" }</h3>
            <Snippet snippet={snippet} />
            <div style="margin-top: 20px;">
                { for keywords.iter().map(|(key, value)| html! {
                    <KeywordInput
                        key_name={key.clone()}
                        value={value.clone()}
                        on_update={update_keyword.clone()}
                    />
                }) }
            </div>
            <div
                style="
                    position: absolute;
                    top: 10px;
                    right: 10px;
                    width: 40px;
                    height: 40px;
                    cursor: pointer;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: red;
                    border-radius: 100%;
                    color: white;
                    font-weight: bold;
                "
                onclick={props.on_close.clone()} // Call the on_close callback when the button is clicked
            >
                { "X" }
            </div>
        </div>
    }
}