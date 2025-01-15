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
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
        justify-content: center;
        align-items: flex-start;
        overflow-x: auto;
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
        width: 20%;
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
                </div>

            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}