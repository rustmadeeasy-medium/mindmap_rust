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
                <Item height = "40px" width = "100px" text = "Basics"/>
                <Line margin_left = "0px" margin_top = "0px" height = "40px"/>
                
                <div style="display: flex; align-items: center; justify-content: center; text-align: center; gap: 10px;">
                    <div style="background-color: black; width: 300px; height: 2px; margin-right : 75px;"></div>

                    <div style="display : flex; position : absolute;"> 
                        <Item height = "40px" width = "100px" text="Variables"/>
                    </div>
                
                </div> 
            
            </div>

            <div style = "display : flex;"> 
                <Line margin_left = "773px" margin_top = "0px" height = "40px"/>
                <Line margin_left = "75px" margin_top = "0px" height = "40px"/>
                <Line margin_left = "219px" margin_top = "0px" height = "150px"/>
            </div>

            <div style="display: flex; flex-direction: row; align-items: center; justify-content: center; position : absolute; margin-top : -110px; margin-left : 600px;">
                <Textitem text = "let"/>
                <Subitem text="Reference" />
                <Subitem text="Mutability" />
                <Textitem text = ":"/>
                <Subitem text="Type" />
                <Textitem text = "="/>
                <Textitem text = "value"/>
            </div>

            <div style="display: flex; flex-direction: column; margin-left : 980px;">
                <Item height="40px" width="175px" text="Compound Types" />
            </div>

        </div>
    }
}