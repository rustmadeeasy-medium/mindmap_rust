use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text : AttrValue,
}

#[function_component(Item)]
pub fn item(props : &Props) -> Html {
    html! {
        <div 
            style = "width: 100px; border-radius : 5px; height : 40px; padding-top : 5px; background-color : #820303; border : 2px solid black;"
        >           
                <p style = "text-align : center; color : white;"> {props.text.clone()} </p>
        </div>
    }
}