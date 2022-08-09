use crate::hooks::UseKeplrHandle;
use yew::prelude::*;

#[function_component]
pub fn SelectChain() -> Html {
    let keplr = use_context::<UseKeplrHandle>()
        .expect("no keplr provider found. you must wrap your components in an <KeplrProvider/>");

    html! {
        <div>
            <select class="form-select mt-2" style="max-width:200px" id="sel1" name="sellist1">
                <option>{"COSMOS HUB"}</option>
                <option>{"OSMOSIS"}</option>
            </select>    
        </div>
    }
}
