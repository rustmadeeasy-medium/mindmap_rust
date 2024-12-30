use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
}

#[function_component(Textitem)]
pub fn textitem(props: &Props) -> Html {
    html! {
        <div
            style={format!(
                "
                    font-weight: bold;
                    color: #b50202;
                    width: 100px;
                    margin-right: 5px;
                    height: 25px;
                "
            )}
            class="text-center"
        >
            <p class="text-center"> {props.text.clone()} </p>
        </div>
    }
}
