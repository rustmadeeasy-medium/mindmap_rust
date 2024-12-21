use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
            <div>           
                <p style = "color : red"> {"Variables"} </p>
            </div>
            
            <div class = "row"> 
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