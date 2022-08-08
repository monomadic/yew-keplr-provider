use crate::hooks::UseKeplrHandle;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub disconnected: Option<Html>,
}

#[function_component]
pub fn ConnectButton(props: &Props) -> Html {
    let keplr = use_context::<UseKeplrHandle>()
        .expect("no Keplr keplr found. you must wrap your components in an <keplr/>");

    let connect = {
        let keplr = keplr.clone();
        use_async(async move { keplr.connect().await })
    };

    let disconnect = {
        let keplr = keplr.clone();
        Callback::from(move |_| keplr.disconnect())
    };

    let disconnected_html = props.disconnected.clone().unwrap_or_else(|| {
        html! {
            <button>{"Disconnect"}</button>
        }
    });

    html! {
        <div>
            if keplr.connected() {
                <div onclick={disconnect}>
                    {disconnected_html}
                </div>
            } else {
                <div onclick={ Callback::from(move |_| connect.run()) }>
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}
