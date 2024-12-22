use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text : AttrValue,
    pub color : AttrValue,
}

#[function_component(Subitem)]
pub fn subitem(props : &Props) -> Html {
    html! {
        <div 
            style = "width: 100px; margin-top : 20px; margin-right : 5px; height : 25px;"  
            class={format!("text-white text-center {}", props.color.clone())}
        >           
                <p class = "text-center"> {props.text.clone()} </p>
        </div>
    }
}