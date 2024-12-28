use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text : AttrValue,
    pub height : AttrValue, 
    pub width : AttrValue, 
}

#[function_component(Item)]
pub fn item(props : &Props) -> Html {
    html! {
        <div 
            style = {format!("padding : auto 10px auto 10px; border-radius : 5px; height : {}; width : {}; background-color : #820303; border : 2px solid black;", props.height.clone(), props.width.clone())}
        >           
                <p style = "margin-top : 5px; text-align : center; color : white;"> {props.text.clone()} </p>
        </div>
    }
}