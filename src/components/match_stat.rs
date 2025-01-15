use yew::prelude::*;

// create the reusable sub-components with styling : snippet, keywords
// include them here, with the right props

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_close: Callback<MouseEvent>, // Change to accept MouseEvent
}

#[function_component(MatchStat)]
pub fn match_stat(props: &Props) -> Html {
    html! {
        <div
            style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                background-color: rgba(255, 255, 255, 0.9); /* Semi-transparent background */
                border: 1px solid black;
                box-sizing: border-box;
                z-index: 9999;
            "
        >
            <div
                style="
                    position: absolute;
                    top: 10px;
                    right: 10px;
                    width: 40px;
                    height: 40px;
                    cursor: pointer;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: red;
                    border-radius: 100%;
                "
                onclick={props.on_close.clone()} // Call the on_close callback when the button is clicked
            >
                <p style="cursor : pointer; margin: 0; color: white; font-weight : bold;">{ "x" }</p>
            
            </div>
            <p>{ "I changed the text" }</p>
        </div>
    }
}