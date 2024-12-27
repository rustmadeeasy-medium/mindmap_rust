use yew::prelude::*;
mod components;

use crate::components::item::Item;
use crate::components::subitem::Subitem;
use crate::components::text_item::Textitem;
use crate::components::line::Line;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
            <div style = "display : flex; flex-direction : column; align-items : center"
            >
                <Item text = "Basics" color = "bg-dark"/>
                <Line margin_left = "0px" margin_top = "0px" />
                <Item text = "Variables" color = "bg-dark"/>
            </div>
            <div style="display: flex; flex-direction: row; align-items: center; justify-content: center;">
                <Textitem text = "let"/>
                <Subitem text="Reference" color="bg-secondary" />
                <Subitem text="Mutability" color="bg-secondary" />
                <Textitem text = ":"/>
                <Subitem text="Type" color="bg-secondary" />
                <Textitem text = "="/>
                <Textitem text = "value"/>
            </div>
        </div>
    }
}