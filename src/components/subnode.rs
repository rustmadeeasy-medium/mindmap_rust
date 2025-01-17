use yew::prelude::*;

// Import the Ownership component
use crate::components::ownership::Ownership;
use crate::components::shadowing::Shadowing;
use crate::components::functions::Functions;
use crate::components::cond_struct::CondStruct;
use crate::components::match_stat::MatchStat;
use crate::components::loop_stat::LoopStat;
use crate::components::while_loop::WhileLoop;
use crate::components::for_loop::ForLoop;


// Default component if no match
fn default_page() -> Html {
    html! { <div>{ "This is the Default Page" }</div> }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub page_to_display: AttrValue,
}

#[function_component(Subnode)]
pub fn subnode(props: &Props) -> Html {
    let subnode_style = "
        background-color: white;
        border: 2px solid rgb(45, 0, 0);
        border-radius: 6px;
        margin-top: 10px;
        padding: 10px;
        text-align: center;
        cursor: pointer;
        color: rgb(45, 0, 0);
        transition: background-color 0.3s ease, transform 0.3s ease;
        width: 80%;
        font-weight : 500;
    ";

    let show_page = use_state(|| false);

    let handle_click = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(true);
        })
    };

    let handle_close = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(false);
        })
    };

    // Use a match statement to choose the component to render
    let page_renderer = match props.page_to_display.as_str() {
        "Ownership" => html! { <Ownership on_close={handle_close.clone()} /> }, // Pass the on_close callback
        "Shadowing" => html! { <Shadowing on_close={handle_close.clone()} /> },
        "Functions" => html! { <Functions on_close={handle_close.clone()} /> },
        "If" => html! { <CondStruct on_close={handle_close.clone()} /> },
        "Match" => html! { <MatchStat on_close={handle_close.clone()} /> },
        "Loop" => html! { <LoopStat on_close={handle_close.clone()} /> },
        "For" => html! { <ForLoop on_close={handle_close.clone()} /> },
        "While" => html! { <WhileLoop on_close={handle_close.clone()} /> },
        _ => default_page(),  // Default component
    };

    html! {
        <>
            <div style={subnode_style} onclick={handle_click}>
                { &props.text }
            </div>
            if *show_page {
                <div>
                    { page_renderer }
                </div>
            }
        </>
    }
}