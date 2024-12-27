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
            style = "width: 100px;"  
            class={format!("{}", props.color.clone())}
        >           
                <p style = "text-align : center; color : white;"> {props.text.clone()} </p>
        </div>
    }
}