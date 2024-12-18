use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
            <div class="var-container">
                <p> {"Variables"} </p>
            </div>
            <div class = "var-content"> 
                <p> {"let"} </p> 
                <div class = "ref"> {"Reference"} </div>
                <div class = "mut"> {"Mutability"} </div>
                <div class = "type"> {"Type"} </div>
            </div>
            <p> {"="} </p>
            <p> {"value"} </p>
        </div>
    }
}