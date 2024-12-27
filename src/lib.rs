use yew::prelude::*;
mod components;

use crate::components::item::Item;
use crate::components::subitem::Subitem;
use crate::components::text_item::Textitem;
use crate::components::line::Line;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div style = "margin-top : 20px;"> 
            
            <div style = "display : flex; flex-direction : column; justify-content : center; align-items : center;">
                <Item text = "Basics" color = "bg-dark"/>
                <Line margin_left = "0px" margin_top = "0px" />
                
                <div style="display: flex; align-items: center; justify-content: center; text-align: center; gap: 10px;">
                    <div style="background-color: black; width: 300px; height: 1px; margin-right : 75px;"></div>

                    <div style="display : flex; position : absolute;"> 
                        <Item text="Variables" color="bg-dark" />
                    </div>
                
                </div> 
            
            </div>

            <Line margin_left = "773px" margin_top = "0px" />

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