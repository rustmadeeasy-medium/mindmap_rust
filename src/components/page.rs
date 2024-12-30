use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |event: MouseEvent| {
            // Restore scrolling on the body when the page is closed
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(body) = document.body() {
                    let body: HtmlElement = body.dyn_into().unwrap();
                    // Set the body to allow scrolling (overflow: auto)
                    body.set_attribute("style", "overflow: auto;").unwrap();
                }
            }
            on_close.emit(event);
        })
    };

    // Disable scrolling on the body when the component is rendered
    if let Some(document) = window().and_then(|w| w.document()) {
        if let Some(body) = document.body() {
            let body: HtmlElement = body.dyn_into().unwrap();
            // Set the body to prevent scrolling (overflow: hidden)
            body.set_attribute("style", "overflow: hidden;").unwrap();
        }
    }

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
            <p>{ "Rewrite it in Rust!" }</p>
            <div
                style="
                    margin-top: 20px;
                    border: 1px solid black;
                    width: 40px;
                    height: 40px;
                    cursor: pointer;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: red;
                "
                onclick={on_close}
            >
                <p style="margin: 0; color: white;">{ "X" }</p>
            </div>
        </div>
    }
}
