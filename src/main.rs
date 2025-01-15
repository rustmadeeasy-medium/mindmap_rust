use yew::prelude::*;

mod components;

use crate::components::page::Page;

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
        margin-bottom: 30px;
        margin-left: auto;
        margin-right: auto;
        text-align: center;
        width: 50%;
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

    let subnode_style = "
        background-color: white;
        border: 1px solid rgb(45, 0, 0);
        border-radius: 6px;
        margin-top: 10px;
        padding: 10px;
        text-align: center;
        cursor: pointer;
        color: rgb(45, 0, 0);
        transition: background-color 0.3s ease, transform 0.3s ease;
        width: 80%;
    ";

    let show_page = use_state(|| false);

    // Handle showing the <Test /> component
    let handle_click = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(true);
        })
    };

    // Handle hiding the <Test /> component (triggered by the child component)
    let hide_page = {
        let show_page = show_page.clone();
        Callback::from(move |_| {
            show_page.set(false);
        })
    };


    let render_subnode = |id: &str, text: &str, additional_class: Option<&str>| -> Html {
        
        let class_name = match additional_class {
            Some(extra) => format!("subnode {}", extra),
            None => "subnode".to_string(),
        };

        html! {
            <div id={id.to_string()} class={class_name} style={subnode_style} onclick = {handle_click.clone()}>
                { text }
            </div>
        }
        
    };

    html! {
        <div>

            <div style={central_node_style}>
                <h2>{ "The Rust Mindmap" }</h2>
            </div>

            <div style={mindmap_style}>

                <div class="node" style={node_style}>
                    <h3>{ "Memory Management" }</h3>
                    { render_subnode("1", "Ownership & Borrowing", Some("snippet-container")) }
                    { render_subnode("2", "Shadowing", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Control Flow" }</h3>
                    { render_subnode("3", "Defining functions", None) }
                    { render_subnode("4", "Parameters & return values", None) }
                    { render_subnode("5", "if / else if / else", None) }
                    { render_subnode("6", "match", None) }
                    { render_subnode("7", "loop", None) }
                    { render_subnode("8", "while", None) }
                    { render_subnode("9", "for", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Primitive Types" }</h3>
                    { render_subnode("10", "Integers", None) }
                    { render_subnode("11", "bool", None) }
                    { render_subnode("12", "chars", None) }
                    { render_subnode("13", "floats", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Collections" }</h3>
                    { render_subnode("14", "arrays", None) }
                    { render_subnode("15", "vectors", None) }
                    { render_subnode("16", "tuples", None) }
                    { render_subnode("17", "HashMap", None) }
                    { render_subnode("18", "HashSet", None) }
                    { render_subnode("19", "BTreeMap", None) }
                    { render_subnode("20", "BTreeSet", None) }
                    { render_subnode("21", "BinaryHeap", None) }
                    { render_subnode("22", "VecDeque", None) }
                </div>

                <div class="node" style = {format!("{} margin-top: -340px; ", node_style)}>
                    <h3>{ "CONST & STATIC" }</h3>
                    { render_subnode("23", "const", None) }
                    { render_subnode("24", "static", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -90px; ", node_style)}>
                    <h3>{ "Custom Types" }</h3>
                    { render_subnode("25", "struct", None) }
                    { render_subnode("26", "enum", None) }
                    { render_subnode("27", "impl", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -240px; ", node_style)}>
                    <h3>{ "Generics & Traits" }</h3>
                    { render_subnode("28", "Generics", None) }
                    { render_subnode("29", "Traits", None) }
                    { render_subnode("30", "Traits with generics", None) }
                    { render_subnode("31", "From Trait", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Option & Result" }</h3>
                    { render_subnode("32", "Pattern Matching", None) }
                    { render_subnode("33", "Option", None) }
                    { render_subnode("34", "Result", None) }
                    { render_subnode("35", "if let", None) }
                    { render_subnode("36", "let else", None) }
                    { render_subnode("37", "while let", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -540px; ", node_style)}>
                    <h3>{ "Error Handling" }</h3>
                    { render_subnode("38", "Panic!", None) }
                    { render_subnode("39", ".unwrap()", None) }
                    { render_subnode("40", ".expect()", None) }
                    { render_subnode("41", ".unwrap_or()", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -240px; ", node_style)}>
                    <h3>{ "Debugging" }</h3>
                    { render_subnode("42", "dbg!", None) }
                    { render_subnode("43", ".inspect()", None) }
                    { render_subnode("44", "Assertions", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -240px; ", node_style)}>
                    <h3>{ "Functional Programming" }</h3>
                    { render_subnode("45", "Closures", None) }
                    { render_subnode("46", "Mapping & Filtering", None) }
                    { render_subnode("47", "Fn types", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "Smart Pointers" }</h3>
                    { render_subnode("48", "Rc", None) }
                    { render_subnode("49", "Arc", None) }
                    { render_subnode("50", "Box", None) }
                    { render_subnode("51", "Interior Mutability", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -540px; ", node_style)}>
                    <h3>{ "Concurrency & Multithreading" }</h3>
                    { render_subnode("52", "Multiple Threads", None) }
                    { render_subnode("53", "Scoped Threads", None) }
                    { render_subnode("54", "channel", None) }
                    { render_subnode("55", "async Rust", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -290px; ", node_style)}>
                    <h3>{ "Memory Management in Concurrency" }</h3>
                    { render_subnode("56", "FnOnce in concurrency", None) }
                    { render_subnode("57", "Rc & Arc in multithreading", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -290px; ", node_style)}>
                    <h3>{ "Macros" }</h3>
                    { render_subnode("58", "Declarative macros", None) }
                    { render_subnode("59", "Procedural macros", None) }
                </div>

                <div class="node" style={node_style}>
                    <h3>{ "External Libraries" }</h3>
                    { render_subnode("60", "serde", None) }
                    { render_subnode("61", "rayon", None) }
                    { render_subnode("62", "reqwest", None) }
                </div>

                <div class="node" style={format!("{} margin-top: -320px; ", node_style)}>
                    <h3>{ "Advanced Concepts" }</h3>
                    { render_subnode("63", "time & chrono", None) }
                    { render_subnode("64", "tokio-runtime", None) }
                </div>

            </div>

            if *show_page {
                <Page on_close={hide_page} />
            }

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
