use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub margin_left : AttrValue,
    pub margin_top : AttrValue,
}

#[function_component(Line)]
pub fn line(props : &Props) -> Html {
    html! {
        <div 
            style = {format!("border : 1px solid black; width: 1px; height : 40px ; margin-left : {}; margin-top : {}", props.margin_left.clone(), props.margin_top.clone())}>           
        </div>
    }
}