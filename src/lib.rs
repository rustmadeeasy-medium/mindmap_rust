use yew::prelude::*;
mod components;

use crate::components::item::Item;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
    
            <Item text = "Basics" color = "bg-dark"/>

            <Item text = "Variables" color = "bg-dark"/>

            
            <div class = "row no-gutters"> 
                <p class = "col-1"> {"let"} </p> 
                <div class = "col-1"> {"Reference"} </div>
                <div class = "col-1"> {"Mutability"} </div>
                <div class = "col-1"> {"Type"} </div>
                <p class = "col-1"> {"="} </p>
                <p class = "col-1"> {"value"} </p>
            </div>
            
        </div>
    }
}