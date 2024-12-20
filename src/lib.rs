use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div> 
            <div>           
                <p style = "color : red"> {"Variables"} </p>
            </div>
            
            <div 
                style = 
                    "display : flex;
                    flex-direction : row;"
            > 
                <p> {"let"} </p> 
                <div class = "ref"> {"Reference"} </div>
                <div class = "mut"> {"Mutability"} </div>
                <div class = "type"> {"Type"} </div>
                <p> {"="} </p>
                <p> {"value"} </p>
            </div>
            
        </div>
    }
}