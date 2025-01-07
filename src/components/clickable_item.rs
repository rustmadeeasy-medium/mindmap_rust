use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::components::page::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub height: AttrValue,
    pub width: AttrValue,
}

#[function_component(ClickableItem)]
pub fn clickable_item(props: &Props) -> Html {
    let show_page = use_state(|| false);

    let handle_click = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(true);
        })
    };

    let hide_page = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(false);
        })
    };

    html! {
        <>
            <div
                onclick={handle_click}
                style={format!(
                    "padding: auto 10px auto 10px; \
                     border-radius: 5px; \
                     height: {}; \
                     width: {}; \
                     background-color: #820303; \
                     margin-bottom : 20px; \
                     border: 2px solid black; \
                     cursor: pointer;", // Added this line
                    props.height.clone(),
                    props.width.clone()
                )}
            >
                <p style="
                    margin-top: 5px;
                    text-align: center;
                    color: white;
                ">
                    {props.text.clone()}
                </p>
            </div>

            if *show_page {
                <Page on_close={hide_page} />
            }
        </>
    }
}
