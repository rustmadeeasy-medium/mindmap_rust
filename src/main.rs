use yew::prelude::*;
mod components;

use crate::components::item::Item;
use crate::components::subitem::Subitem;
use crate::components::text_item::Textitem;
use crate::components::line::Line;
use crate::components::clickable_item::ClickableItem;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div
            style="
                margin-top: 20px;
                overflow: hidden;
            "
        > 
            <div
                style="
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                "
            >
                <Item height="40px" width="100px" text="Basics" />
                <Line margin_left="0px" margin_top="0px" height="40px" />
                
                <div
                    style="
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        text-align: center;
                        gap: 10px;
                    "
                >
                    <div
                        style="
                            background-color: black;
                            width: 300px;
                            height: 2px;
                            margin-right: 75px;
                        "
                    ></div>
                    <div
                        style="
                            display: flex;
                            position: absolute;
                        "
                    > 
                        <Item height="40px" width="100px" text="Variables" />
                    </div>
                </div>
            
            </div>

            <div
                style="
                    display: flex;
                "
            > 
                <Line margin_left="773px" margin_top="0px" height="40px" />
                <Line margin_left="75px" margin_top="0px" height="40px" />
                <Line margin_left="219px" margin_top="0px" height="180px" />
            </div>

            <div
                style="
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;
                    position: absolute;
                    margin-top: -150px;
                    margin-left: 600px;
                "
            >
                <Textitem text="let" />
                <Subitem text="Reference" />
                <Subitem text="Mutability" />
                <Textitem text=":" />
                <Subitem text="Type" />
                <Textitem text="=" />
                <Textitem text="value" />
            </div>

            <div
                style="
                    display: flex;
                    flex-direction: column;
                    margin-left: 990px;
                    margin-top: -40px;
                "
            >
                <Item height="40px" width="160px" text="Custom Types" />
                <Line margin_left="5px" margin_top="0px" height="100px" />
                
                <div
                    style="
                        display: flex;
                    "
                > 
                    <div
                        style="
                            background-color: black;
                            width: 40px;
                            height: 2px;
                            margin-left: 5px;
                            margin-top: -60px;
                        "
                    ></div>

                    <div
                        style="
                            background-color: black;
                            width: 40px;
                            height: 2px;
                            margin-left : -40px;
                        "
                    ></div>
                    
                    <div
                        style="
                            display: flex;
                            flex-direction: column;
                            margin-top: -80px;
                        "
                    >
                        <ClickableItem height="40px" width="100px" text="struct" />
                        <ClickableItem height="40px" width="100px" text="enum" />


                    </div>
                </div>
            
            </div>

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}