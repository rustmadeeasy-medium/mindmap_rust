use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text : AttrValue,
}

#[function_component(Subitem)]
pub fn subitem(props : &Props) -> Html {
    html! {
        <div 
            style = "width: 100px; margin-right : 5px; height : 30px; border-radius : 5px; border : 2px solid black; background-color : #b50202;"  
            class={format!("text-white text-center")}
        >           
                <p class = "text-center"> {props.text.clone()} </p>
        </div>
    }
}