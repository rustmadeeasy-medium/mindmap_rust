use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::components::page::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
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
                    "
                        background-color: white;
                        border: 1px solid rgb(45, 0, 0);
                        border-radius: 6px;
                        margin-top: 10px;
                        padding-top : 10px;
                        text-align: center;
                        cursor: pointer;
                        color: rgb(45, 0, 0);
                        transition: background-color 0.3s ease, transform 0.3s ease;
                        width: 80%;
                    ", 
                )}
            >
                <p>
                    {props.text.clone()}
                </p>
            </div>

            if *show_page {
                <Page on_close={hide_page} />
            }
        </>
    }
}
