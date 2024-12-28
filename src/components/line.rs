use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub margin_left : AttrValue,
    pub margin_top : AttrValue,
    pub height : AttrValue,
}

#[function_component(Line)]
pub fn line(props : &Props) -> Html {
    html! {
        <div 
            style = {format!("border : 1px solid black; width: 1px; margin-left : {}; margin-top : {}; height : {}", props.margin_left.clone(), props.margin_top.clone(), props.height.clone())}>           
        </div>
    }
}