use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text : AttrValue,
    pub color : AttrValue,
}

#[function_component(Item)]
pub fn item(props : &Props) -> Html {
    html! {
        <div 
            style = "width: 100px; margin : auto; margin-top : 20px;"  
            class={format!("text-white text-center {}", props.color.clone())}
        >           
                <p class = "text-center"> {props.text.clone()} </p>
        </div>
    }
}