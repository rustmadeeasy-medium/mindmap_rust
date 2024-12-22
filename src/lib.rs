use yew::prelude::*;
mod components;

use crate::components::item::Item;
use crate::components::subitem::Subitem;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
    
            <Item text = "Basics" color = "bg-dark"/>
            <Item text = "Variables" color = "bg-dark"/>

            
            <div class = "row no-gutters" style = "margin : auto; align-items : center; justify-content : center"> 
                <Subitem text = "let" color = "bg-secondary"/>
                <Subitem text = "Reference" color = "bg-secondary"/>
                <Subitem text = "Mutability" color = "bg-secondary"/>
                <Subitem text = "Type" color = "bg-secondary"/>
                <Subitem text = "=" color = "bg-secondary"/>
                <Subitem text = "value" color = "bg-secondary"/>
            </div>
            
        </div>
    }
}