use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
            <h1> {"Hello World !! "} </h1>
            <h2> {"I love Rust "} </h2> 
        </div>
    }
}