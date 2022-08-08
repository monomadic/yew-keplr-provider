use crate::hooks::UseKeplrHandle;
use yew::prelude::*;

#[function_component]
pub fn AccountLabel() -> Html {
    let keplr = use_context::<UseKeplrHandle>()
        .expect("no keplr provider found. you must wrap your components in an <KeplrProvider/>");

    html! {
        <div>
            if keplr.connected() {
                {keplr.display_address()}
            } else {
                {"disconnected"}
            }
        </div>
    }
}
