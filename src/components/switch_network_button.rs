use crate::hooks::UseKeplrHandle;
use wasm_bindgen_futures::spawn_local;
use web3::transports::eip_1193::Chain;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chain: Chain,

    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component]
pub fn SwitchNetworkButton(props: &Props) -> Html {
    let _keplr = use_context::<UseKeplrHandle>()
        .expect("no keplr provider found. you must wrap your components in an <Keplr/>");

    let _chain = props.chain.clone();

    let on_click = {
        Callback::from(move |_| {
            //let keplr = keplr.clone();
            //let chain = chain.clone();
            spawn_local(async move {
                //let _ = keplr.switch_chain_with_fallback(chain).await;
                //let _ = keplr.switch_chain_with_fallback(&chain).await;
            });
        })
    };

    html! {
        <div>
            <button onclick={on_click} class={&props.class}>
                {"Switch to "}{&props.chain.chain_name}
            </button>
        </div>
    }
}
