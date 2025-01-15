use yew::prelude::*;

mod components;

use crate::components::subnode::Subnode;

#[function_component(App)]
pub fn app() -> Html {
    let central_node_style = "
        background-color: rgb(132, 30, 1);
        border: 1px solid rgb(45, 0, 0);
        color: white;
        font-size: 2rem;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        margin : 20px auto 30px auto;
        text-align: center;
        width: 25%;
    ";

    let mindmap_style = "
        display: grid;
        grid-template-columns : repeat(5, 1fr);
        grid-template-rows : masonry;
        justify-items: center;
        align-items : start;
        overflow-x: auto;
        margin-left : 20px;
        margin-right : 20px;
    ";

    let node_style = "
        background-color: #ff5e016a;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        padding: 20px;
        margin: 10px 0;
        text-align: center;
        position: relative;
        border: 1px solid rgb(45, 0, 0);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 350px;
    ";

    html! {
        <div>

            <div style={central_node_style}>
                <h2>{ "The Rust Mindmap" }</h2>
            </div>

            <div style={mindmap_style}>

                <div class="node" style={node_style}>
                    <h3>{ "Memory Management" }</h3>
                    <Subnode text = "Ownership & Borrowing" page_to_display = "Ownership"/>
                    <Subnode text = "Shadowing" page_to_display = "Shadowing"/>
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Control Flow" }</h3>
                    <Subnode text = "Functions" page_to_display = "Functions"/>
                    <Subnode text = "if / else if / else" page_to_display = "If"/>
                    <Subnode text = "match" page_to_display = "Match"/>
                    <Subnode text = "loop" page_to_display = "Loop"/>
                    <Subnode text = "for" page_to_display = "For"/>
                    <Subnode text = "while" page_to_display = "While"/>
                </div>

            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}